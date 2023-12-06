namespace LumeHub.Api.Example;

public sealed class Endpoint : Endpoint<Request, Response, Mapper>
{
    public override void Configure() => Get("example");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        var entity = Map.ToEntity(req);
        await SendAsync(Map.FromEntity(entity), (int)HttpStatusCode.OK, ct);
    }
}