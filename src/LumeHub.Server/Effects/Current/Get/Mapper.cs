namespace LumeHub.Server.Effects.Current.Get;

public sealed class Mapper : ResponseMapper<Response, EffectDto>
{
    public override Response FromEntity(EffectDto e) => new()
    {
        Id = e.Id,
        Name = e.Name,
        Data = e.Data
    };
}