using LumeHub.Core.Effects;
using LumeHub.Core.LedControl;
using System.ComponentModel.DataAnnotations;

namespace LumeHub.Server.Effects;

public class EffectDto
{
    [Key]
    [MaxLength(36)]
    public string Id { get; init; } = Guid.NewGuid().ToString();
    public required string Name { get; set; }
    public required string Data { get; set; }

    private Effect? _effect;
    public Effect Effect => _effect ??= this!;

    public void Apply(LedController ledController) => Effect.Apply(ledController);

    public static implicit operator Effect?(EffectDto dto)
    {
        EffectUtils.TryConvert(dto.Data, out var effect);
        return effect;
    }
}