using LumeHub.Core.Effects;

namespace LumeHub.Server.Effects.Update;

public sealed class Endpoint(IRepository repository) : Endpoint<Request>
{
    public override void Configure() => Put("effects/{id}");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!repository.Exists(req.Id))
        {
            Logger.LogInformation("Could not find effect with id {Id}.", req.Id);
            await SendNotFoundAsync(ct);
            return;
        }

        if (!EffectUtils.TryConvert(req.Data, out _))
        {
            Logger.LogWarning("The provided json data cannot be converted into a valid effect.");
            ThrowError(r => r.Data, "The provided json data cannot be converted into a valid effect.");
        }

        var effect = repository.Get(req.Id);
        effect.Name = req.Name;
        effect.Data = req.Data;
        repository.Update(effect);
        Logger.LogInformation("Successfully update effect with id {Id}.", req.Id);
        await SendNoContentAsync(ct);
    }
}