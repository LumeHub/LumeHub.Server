using LumeHub.Api.Effects;

namespace LumeHub.Api.State.Get;

public sealed class Endpoint(IManager manager) : EndpointWithoutRequest<Response>
{
    public override void Configure() => Get("state");

    public override async Task HandleAsync(CancellationToken ct)
    {
        Logger.LogInformation("Successfully returned the state of the led strip.");
        await SendAsync(new Response { IsOn = manager.IsOn, }, cancellation: ct);
    }
}