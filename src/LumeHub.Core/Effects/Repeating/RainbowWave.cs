using LumeHub.Core.Colors;
using LumeHub.Core.LedControl;

namespace LumeHub.Core.Effects.Repeating;

public sealed class RainbowWave() : RepeatingEffect(nameof(RainbowWave))
{
    public required float Multiplier { get; init; }
    public required int Timeout { get; init; }

    protected override void Update(LedController ledController, CancellationToken ct)
    {
        for (int colorIndex = 0; colorIndex < 256; colorIndex++)
        {
            for (int i = 0; i < ledController.PixelCount; i++)
            {
                if (ct.IsCancellationRequested) break;
                ledController[i] = new RainbowCycleColor(colorIndex, ledController.PixelCount, Multiplier, i);
            }

            if (ct.IsCancellationRequested) break;

            ledController.Show();
            Thread.Sleep(Timeout);
        }
    }
}