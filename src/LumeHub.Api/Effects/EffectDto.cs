using LumeHub.Core.Effects;

namespace LumeHub.Api.Effects;

public class EffectDto
{
    public string Id { get; init; } = Guid.NewGuid().ToString();
    public required string Name { get; init; }
    public required string Data { get; init; }

    public static implicit operator Effect?(EffectDto dto)
    {
        EffectUtils.TryConvert(dto.Data, out var effect);
        return effect;
    }
}
