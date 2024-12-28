using LumeHub.Core.Colors;
using System.Drawing;

namespace LumeHub.Core.UnitTests.Colors;

public sealed class RgbColorFacts
{
    [Theory]
    [InlineData(255, 0, 0)]
    [InlineData(0, 255, 0)]
    [InlineData(0, 0, 255)]
    [InlineData(128, 128, 0)]
    [InlineData(0, 128, 128)]
    public void ImplicitOperator_FromRgbColorToColor_ShouldConvertCorrectly(byte red, byte green, byte blue)
    {
        // Arrange
        var rgbColor = new RgbColor(red, green, blue);

        // Act
        Color result = rgbColor;

        // Assert
        result.R.Should().Be(red);
        result.G.Should().Be(green);
        result.B.Should().Be(blue);
    }

    [Theory]
    [InlineData(255, 0, 0)]
    [InlineData(0, 255, 0)]
    [InlineData(0, 0, 255)]
    [InlineData(128, 128, 0)]
    [InlineData(0, 128, 128)]
    public void ImplicitOperator_FromColorToRgbColor_ShouldConvertCorrectly(byte red, byte green, byte blue)
    {
        // Arrange
        var color = Color.FromArgb(red, green, blue);

        // Act
        RgbColor result = color;

        // Assert
        result.Red.Should().Be(red);
        result.Green.Should().Be(green);
        result.Blue.Should().Be(blue);
    }
}