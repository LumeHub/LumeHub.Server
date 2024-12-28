using LumeHub.Core.Colors;
using LumeHub.Core.LedControl;

namespace LumeHub.Core.Effects.Normal;

public class FadeColor() : Effect(nameof(FadeColor))
{
    public required RgbColor Color { get; init; }

    protected override void Execute(LedController ledController, CancellationToken ct)
    {
        var currentColor = ledController[0];
        foreach (var color in RgbColorUtils.InterpolateColors(currentColor, Color, 5))
        {
            if (ct.IsCancellationRequested) return;

            ledController.SetAllPixel(color);
            ledController.Show();
            Thread.Sleep(10);
        }
    }
}