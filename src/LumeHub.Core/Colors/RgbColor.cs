using System.Drawing;

namespace LumeHub.Core.Colors;

public record RgbColor(byte Red, byte Green, byte Blue)
{
    public RgbColor(int red, int green, int blue) : this((byte)red, (byte)green, (byte)blue)
    { }

    public RgbColor() : this(0, 0, 0)
    { }

    public static implicit operator Color(RgbColor rgbColor)
    {
        return Color.FromArgb(rgbColor.Red, rgbColor.Green, rgbColor.Blue);
    }

    public static implicit operator RgbColor(Color color)
    {
        return new RgbColor(color.R, color.G, color.B);
    }

    public float GetDistance(RgbColor other) =>
        MyMath.Max(Math.Abs(Red - other.Red), Math.Abs(Green - other.Green), Math.Abs(Blue - other.Blue));
}