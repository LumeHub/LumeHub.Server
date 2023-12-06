namespace LumeHub.Api.Example;

public sealed class Mapper : Mapper<Request, Response, Model>
{
    public override Response FromEntity(Model e) => new()
    {
        String = e.String
    };

    public override Model ToEntity(Request r) => new()
    {
        String = r.String
    };
}