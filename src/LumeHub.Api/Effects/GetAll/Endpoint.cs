namespace LumeHub.Api.Effects.GetAll;

public sealed class Endpoint(IRepository repository) : EndpointWithoutRequest<Response, Mapper>
{
    public override void Configure() => Get("effects");

    public override async Task HandleAsync(CancellationToken ct)
    {
        var effects = repository.GetAll();
        var response = Map.FromEntity(effects);
        Logger.LogInformation("Successfully returned {Count} effects.", response.Effects.Count());
        await SendAsync(response, cancellation: ct);
    }
}