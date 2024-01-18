using LumeHub.Core.Effects;

namespace LumeHub.Api.Effects.Add;

public sealed class Endpoint(IRepository repository) : Endpoint<Request, Response, Mapper>
{
    public override void Configure() => Post("effects");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!EffectUtils.TryConvert(req.Data, out _))
        {
            Logger.LogWarning("The provided json data cannot be converted into a valid effect.");
            ThrowError(r => r.Data, "The provided json data cannot be converted into a valid effect.");
        }
        
        string id = repository.Add(Map.ToEntity(req)!);
        Logger.LogInformation("Successfully added an effect ith the id {Id}.", id);
        var response = new Response { Id = id };
        await SendAsync(response, (int)HttpStatusCode.Created, ct);
    }
}