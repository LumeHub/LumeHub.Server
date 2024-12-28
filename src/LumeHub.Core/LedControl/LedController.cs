using LumeHub.Core.Colors;
using Microsoft.Extensions.Options;

namespace LumeHub.Core.LedControl;

public abstract class LedController
{
    public int PixelCount { get; }

    protected LedController(IOptions<LedControllerOptions> options)
    {
        PixelCount = options.Value.PixelCount;

        SetAllPixel(new RgbColor(0, 0, 0));
    }

    public abstract RgbColor this[int index] { get; set; }

    public void SetAllPixel(RgbColor color)
    {
        for (int i = 0; i < PixelCount; i++) this[i] = color;
    }

    public abstract void Show();
}