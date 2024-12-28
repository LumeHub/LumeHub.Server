using LumeHub.Core.LedControl;
using System.Text.Json.Serialization;

namespace LumeHub.Core.Effects;

[JsonConverter(typeof(EffectConverter))]
public abstract class Effect(string name)
{
    public string Name => name;

    private CancellationTokenSource? _tokenSource;

    public void Apply(LedController ledController)
    {
        _tokenSource = new CancellationTokenSource();
        Execute(ledController, _tokenSource.Token);
    }

    protected abstract void Execute(LedController ledController, CancellationToken ct);

    public void Stop()
    {
        _tokenSource?.Cancel();
        OnStopped();
    }

    protected virtual void OnStopped()
    { }
}