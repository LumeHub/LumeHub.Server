namespace LumeHub.Api.Effects.GetAll;

public sealed class Mapper : ResponseMapper<Response, IEnumerable<EffectDto>>
{
    public override Response FromEntity(IEnumerable<EffectDto> e) => new()
    {
        Effects = e.Select(c => new Response.Effect
        {
            Id = c.Id,
            Name = c.Name,
            Data = c.Data
        })
    };
}