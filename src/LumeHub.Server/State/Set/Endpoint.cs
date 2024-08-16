using LumeHub.Server.Effects;

namespace LumeHub.Server.State.Set;

public sealed class Endpoint(IManager manager) : Endpoint<Request>
{
    public override void Configure() => Post("state");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        manager.Toggle(req.State);
        Logger.LogInformation("Set state of the led strip to {State}.", req.State);
        await SendNoContentAsync(ct);
    }
}