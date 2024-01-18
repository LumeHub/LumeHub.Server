namespace LumeHub.Api.Effects.Remove;

public sealed class Endpoint(IRepository repository) : Endpoint<Request>
{
    public override void Configure() => Delete("effects/{id}");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!repository.Exists(req.Id))
        {
            Logger.LogInformation("Could not find effect with id {Id}.", req.Id);
            await SendNotFoundAsync(ct);
            return;
        }

        repository.Remove(req.Id);
        Logger.LogInformation("Successfully removed effect with id {Id}.", req.Id);
        await SendNoContentAsync(ct);
    }
}