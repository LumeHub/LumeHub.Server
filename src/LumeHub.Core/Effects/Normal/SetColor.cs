using LumeHub.Core.Colors;
using LumeHub.Core.LedControl;

namespace LumeHub.Core.Effects.Normal;

public sealed class SetColor() : Effect(nameof(SetColor))
{
    public required RgbColor Color { get; init; }

    protected override void Execute(LedController ledController, CancellationToken ct)
    {
        ledController.SetAllPixel(Color);
        ledController.Show();
    }
}