using LumeHub.Core.Colors;
using LumeHub.Core.Effects;
using LumeHub.Core.Effects.Normal;
using System.Text.Json;

namespace LumeHub.Server.Effects;

public interface IManager
{
    EffectDto? CurrentEffect { get; }
    bool IsOn { get; }
    void SetEffect(EffectDto effect);
    void UpdateEffect(Effect effect);
    void Toggle(bool state);
}

public sealed class Manager(QueueingService queueingService) : IManager
{
    public EffectDto? CurrentEffect { get; private set; }
    public bool IsOn { get; private set; } = true;
    private readonly FadeColor _offEffect = new() { Color = new RgbColor() };

    public void SetEffect(EffectDto effect)
    {
        ApplyEffect(effect.Effect);
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
        if (state == IsOn) return;
        if (CurrentEffect is null) return;

        if (state) ApplyEffect(CurrentEffect.Effect);
        else ApplyEffect(_offEffect);

        IsOn = state;
    }

    private void ApplyEffect(Effect effect)
    {
        CurrentEffect?.Effect.Stop();
        queueingService.Enqueue(effect);
    }
}