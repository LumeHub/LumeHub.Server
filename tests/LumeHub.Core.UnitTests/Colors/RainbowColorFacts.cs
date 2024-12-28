using LumeHub.Core.Colors;

namespace LumeHub.Core.UnitTests.Colors;

public sealed class RainbowColorFacts
{
    [Theory]
    [InlineData(0, 0, 255, 0)]
    [InlineData(42, 126, 129, 0)]
    [InlineData(85, 255, 0, 0)]
    [InlineData(127, 129, 0, 126)]
    [InlineData(170, 0, 0, 255)]
    [InlineData(212, 0, 126, 129)]
    [InlineData(255, 0, 255, 0)]
    public void RainbowColor_CreatesCorrectRgbColor(int colorIndex, byte expectedRed, byte expectedGreen, byte expectedBlue)
    {
        // Act
        var rainbowColor = new RainbowColor(colorIndex);

        // Assert
        rainbowColor.Red.Should().Be(expectedRed);
        rainbowColor.Green.Should().Be(expectedGreen);
        rainbowColor.Blue.Should().Be(expectedBlue);
    }
}