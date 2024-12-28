namespace LumeHub.Core.Colors;

public record RainbowCycleColor : RainbowColor
{
    /// <param name="colorIndex">the index of the color in the rainbow 0 to 255</param>
    /// <param name="pixelCount">the <see cref="LedControl.LedController.PixelCount"/></param>
    /// <param name="frequencyMultiplier">the frequency multiplier to apply to the color cycle</param>
    /// <param name="pixelIndex">the index of the pixel in the strip</param>
    public RainbowCycleColor(int colorIndex, int pixelCount, float frequencyMultiplier, int pixelIndex = 1)
        : base(MyMath.Modulus((pixelIndex * (int)(256f * frequencyMultiplier) / pixelCount) + colorIndex, 256))
    { }
}