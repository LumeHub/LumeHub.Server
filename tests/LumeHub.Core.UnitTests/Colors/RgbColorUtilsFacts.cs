using LumeHub.Core.Colors;

namespace LumeHub.Core.UnitTests.Colors;

public sealed class RgbColorUtilsFacts
{
    [Fact]
    public void InterpolateColors_ReturnsLastColor_WhenColorsAreEqual()
    {
        // Arrange
        var color = new RgbColor(50, 100, 150);

        // Act
        var result = RgbColorUtils.InterpolateColors(color, color, 10);

        // Assert
        var rgbColors = result.ToList();
        rgbColors.Should().HaveCount(1);
        rgbColors[0].Should().Be(color);
    }

    [Fact]
    public void InterpolateColors_ReturnsLastColor_WhenStepCountIs1()
    {
        // Arrange
        var color1 = new RgbColor(0, 0, 0);
        var color2 = new RgbColor(255, 255, 255);

        // Act
        var result = RgbColorUtils.InterpolateColors(color1, color2, 255).ToList();

        // Assert
        result.Should().HaveCount(1);
        result[0].Should().Be(color2);
    }

    [Fact]
    public void InterpolateColors_ReturnsCorrectColors_WhenStepSizeIs50()
    {
        // Arrange
        var color1 = new RgbColor(0, 0, 0);
        var color2 = new RgbColor(255, 255, 255);

        // Act
        var result = RgbColorUtils.InterpolateColors(color1, color2, 50).ToList();

        // Assert
        result.Should().HaveCount(5);
        result.Should().BeEquivalentTo([
            new RgbColor(0, 0, 0),
            new RgbColor(63, 63, 63),
            new RgbColor(127, 127, 127),
            new RgbColor(191, 191, 191),
            new RgbColor(255, 255, 255)
        ]);
    }
}