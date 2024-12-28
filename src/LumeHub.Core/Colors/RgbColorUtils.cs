namespace LumeHub.Core.Colors;

public static class RgbColorUtils
{
    public static IEnumerable<RgbColor> InterpolateColors(RgbColor color1, RgbColor color2, int stepSize)
    {
        int stepCount = (int)Math.Max(color1.GetDistance(color2) / stepSize, 1);
        if (stepCount == 1)
        {
            yield return color2;
            yield break;
        }

        for (int i = 0; i < stepCount; i++)
        {
            int r = (((stepCount - i - 1) * color1.Red) + (i * color2.Red)) / (stepCount - 1);
            int g = (((stepCount - i - 1) * color1.Green) + (i * color2.Green)) / (stepCount - 1);
            int b = (((stepCount - i - 1) * color1.Blue) + (i * color2.Blue)) / (stepCount - 1);
            yield return new RgbColor(r, g, b);
        }
    }
}