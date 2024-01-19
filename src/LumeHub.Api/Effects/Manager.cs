using LumeHub.Core.LedControl;

namespace LumeHub.Api.Effects;

public interface IManager
{
    EffectDto? CurrentEffect { get; }
    void ApplyEffect(EffectDto effect);
}

public sealed class Manager(LedController ledController) : IManager
{
    public EffectDto? CurrentEffect { get; private set; }

    public void ApplyEffect(EffectDto effect)
    {
        if (CurrentEffect is null && CurrentEffect!.IsRepeatingEffect(out var repeatingEffect))
        {
            repeatingEffect!.Stop();
        }

        effect.Apply(ledController);
        CurrentEffect = effect;
    }
}
