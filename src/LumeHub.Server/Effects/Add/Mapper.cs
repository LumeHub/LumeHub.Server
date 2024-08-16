namespace LumeHub.Server.Effects.Add;

public sealed class Mapper : Mapper<Request, Response, EffectDto>
{
    public override EffectDto ToEntity(Request r) => new()
    {
        Name = r.Name,
        Data = r.Data
    };
}