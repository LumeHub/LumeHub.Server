using LumeHub.Core.Colors;
using System.Drawing;

namespace LumeHub.Core.UnitTests.Colors;

public sealed class ColorExtensionFacts
{
    [Theory]
    [InlineData(255, 0, 0, "Red")]
    [InlineData(0, 0, 255, "Blue")]
    [InlineData(0, 255, 0, "Lime")]
    [InlineData(0, 0, 0, "Black")]
    [InlineData(255, 255, 255, "White")]
    [InlineData(255, 255, 0, "Yellow")]
    [InlineData(255, 0, 255, "Fuchsia")]
    [InlineData(0, 255, 255, "Aqua")]
    [InlineData(128, 0, 0, "Maroon")]
    [InlineData(0, 0, 128, "Navy")]
    [InlineData(128, 128, 0, "Olive")]
    [InlineData(128, 0, 128, "Purple")]
    [InlineData(0, 128, 128, "Teal")]
    [InlineData(192, 192, 192, "Silver")]
    [InlineData(128, 64, 0, "SaddleBrown")]
    [InlineData(255, 165, 0, "Orange")]
    [InlineData(255, 192, 203, "Pink")]
    [InlineData(0, 128, 0, "DarkGreen")]
    [InlineData(70, 130, 180, "SteelBlue")]
    [InlineData(50, 205, 50, "LimeGreen")]
    [InlineData(255, 99, 71, "Tomato")]
    [InlineData(0, 139, 139, "DarkCyan")]
    [InlineData(0, 250, 154, "MediumSpringGreen")]
    [InlineData(255, 218, 185, "PeachPuff")]
    [InlineData(0, 0, 139, "DarkBlue")]
    [InlineData(255, 20, 147, "DeepPink")]
    [InlineData(205, 133, 63, "Peru")]
    [InlineData(255, 215, 0, "Gold")]
    public void GetClosestKnownColorName_ShouldReturnClosestColor(byte red, byte green, byte blue, string expectedName)
    {
        // Arrange
        var targetColor = Color.FromArgb(red, green, blue);

        // Act
        string result = targetColor.GetClosestKnownColorName();

        // Assert
        result.Should().Be(expectedName);
    }
}