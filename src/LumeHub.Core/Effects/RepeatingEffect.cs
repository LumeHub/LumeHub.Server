using LumeHub.Core.LedControl;

namespace LumeHub.Core.Effects;

public abstract class RepeatingEffect(string name) : Effect(name)
{
    protected override void Execute(LedController ledController, CancellationToken ct)
    {
        while (!ct.IsCancellationRequested) Update(ledController, ct);
    }

    protected abstract void Update(LedController ledController, CancellationToken ct);
}