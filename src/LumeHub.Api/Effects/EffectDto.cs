using LumeHub.Core.Effects;
using LumeHub.Core.LedControl;
using System.ComponentModel.DataAnnotations;

namespace LumeHub.Api.Effects;

public class EffectDto
{
    [Key]
    public string Id { get; init; } = Guid.NewGuid().ToString();
    public required string Name { get; set; }
    public required string Data { get; set; }

    public void Apply(LedController ledController)
    {
        Effect effect = this!;
        effect.Apply(ledController);
    }

    public bool IsRepeatingEffect(out RepeatingEffect? repeatingEffect)
    {
        repeatingEffect = null;
        if ((Effect)this! is not RepeatingEffect repeating) return false;

        repeatingEffect = repeating;
        return true;
    }

    public static implicit operator Effect?(EffectDto dto)
    {
        EffectUtils.TryConvert(dto.Data, out var effect);
        return effect;
    }
}
