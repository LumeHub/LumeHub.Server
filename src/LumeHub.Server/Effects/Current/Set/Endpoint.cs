using LumeHub.Core.Effects;

namespace LumeHub.Server.Effects.Current.Set;

public sealed class Endpoint(IRepository repository, IManager manager) : Endpoint<Request>
{
    public override void Configure() => Post("effects/current");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!repository.Exists(req.Id))
        {
            Logger.LogInformation("Could not find an effect with id {Id}.", req.Id);
            await SendNotFoundAsync(ct);
            return;
        }

        // Get and apply effect
        var effect = repository.Get(req.Id);
        manager.SetEffect(effect);
        Logger.LogInformation("Successfully applied effect with the id {Id}.", req.Id);
        await SendNoContentAsync(ct);
    }
}