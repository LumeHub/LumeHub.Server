namespace LumeHub.Api.Effects.Get;

public sealed class Mapper : Mapper<Request, Response, EffectDto>
{
    public override Response FromEntity(EffectDto e) => new()
    {
        Id = e.Id,
        Name = e.Name,
        Data = e.Data
    };
}