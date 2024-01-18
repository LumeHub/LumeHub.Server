namespace LumeHub.Api.Effects.Get;

public sealed class Endpoint(IRepository repository) : Endpoint<Request, Response, Mapper>
{
    public override void Configure() => Get("effects/{id}");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!repository.Exists(req.Id))
        {
            Logger.LogInformation("Could not find effect with id {Id}.", req.Id);
            await SendNotFoundAsync(ct);
            return;
        }

        var effect = repository.Get(req.Id);
        Logger.LogInformation("Successfully got effect with id {Id}.", req.Id);
        await SendAsync(Map.FromEntity(effect), cancellation: ct);
    }
}