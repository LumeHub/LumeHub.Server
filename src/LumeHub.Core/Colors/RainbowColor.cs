namespace LumeHub.Core.Colors;

public record RainbowColor : RgbColor
{
    /// <param name="colorIndex">the index of the color in the rainbow 0 to 255</param>
    public RainbowColor(int colorIndex) : base(colorIndex switch
    {
        < 85 => new RgbColor(colorIndex * 3, 255 - (colorIndex * 3), 0),
        < 170 => new RgbColor(255 - ((colorIndex - 85) * 3), 0, (colorIndex - 85) * 3),
        _ => new RgbColor(0, (colorIndex - 170) * 3, 255 - ((colorIndex - 170) * 3))
    })
    { }
}