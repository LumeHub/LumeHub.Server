using LumeHub.Core.Colors;
using LumeHub.Core.Effects;
using LumeHub.Core.Effects.Normal;
using LumeHub.Core.LedControl;
using System.Text.Json;

namespace LumeHub.Api.Effects;

public interface IManager
{
    EffectDto? CurrentEffect { get; }
    bool IsOn { get; }
    void SetEffect(EffectDto effect);
    void UpdateEffect(Effect effect);
    void Toggle(bool state);
}

public sealed class Manager(LedController ledController) : IManager
{
    public EffectDto? CurrentEffect { get; private set; }
    public bool IsOn { get; private set; } = true;
    private readonly FadeColor _offEffect = new() { Color = new RgbColor() };

    public void SetEffect(EffectDto effect)
    {
        ApplyEffect(effect!);
        CurrentEffect = effect;
        IsOn = true;
    }

    public void UpdateEffect(Effect effect)
    {
        ApplyEffect(effect);
        string data = JsonSerializer.Serialize(effect);
        if (CurrentEffect is null)
            CurrentEffect = new EffectDto { Id = "", Name = "", Data = data };
        else
            CurrentEffect.Data = data;
        IsOn = true;
    }

    public void Toggle(bool state)
    {
        if (state) ApplyEffect(_offEffect);
        else CurrentEffect?.Apply(ledController);
        IsOn = state;
    }

    private void ApplyEffect(Effect effect)
    {
        // If CurrentEffect is set and is repeating effect
        if (IsOn && (CurrentEffect?.IsRepeatingEffect(out var repeatingEffect) ?? false))
            repeatingEffect!.Stop();

        effect.Apply(ledController);
    }
}
