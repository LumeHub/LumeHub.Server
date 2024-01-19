namespace LumeHub.Api.Effects.Current.Get;

public sealed class Endpoint(IManager effectManager) : EndpointWithoutRequest<Response, Mapper>
{
    public override void Configure() => Get("effects/current");

    public override async Task HandleAsync(CancellationToken ct)
    {
        var effect = effectManager.CurrentEffect;
        if (effect is null)
        {
            Logger.LogInformation("There was no effect applied yet.");
            await SendNoContentAsync(ct);
            return;
        }

        Logger.LogInformation("Successfully got the current effect.");
        await SendAsync(Map.FromEntity(effect!), cancellation: ct);
    }
}