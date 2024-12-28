using LumeHub.Core.Colors;
using LumeHub.Core.Effects;
using LumeHub.Core.Effects.Normal;

namespace LumeHub.Core.UnitTests.Effects;

public sealed class EffectUtilsFacts
{
    [Fact]
    public void TryConvert_WithValidJson_ShouldReturnTrueAndEffectObject()
    {
        // Arrange
        const string json = """{"Name":"FadeColor","Color":{"Red":255,"Green":0,"Blue":0}}""";

        // Act
        bool result = EffectUtils.TryConvert(json, out var effect);

        // Assert
        result.Should().BeTrue();
        effect.Should().NotBeNull();
        effect!.Name.Should().Be("FadeColor");
        (effect as FadeColor)!.Color.Should().BeEquivalentTo(new RgbColor(255, 0, 0));
    }

    [Fact]
    public void TryConvert_WithInvalidJson_ShouldReturnFalseAndNullEffectObject()
    {
        // Arrange
        const string json = "invalid json";

        // Act
        bool result = EffectUtils.TryConvert(json, out var effect);

        // Assert
        result.Should().BeFalse();
        effect.Should().BeNull();
    }

    [Fact]
    public void TryConvert_WithNullJson_ShouldReturnFalseAndNullEffectObject()
    {
        // Arrange
        string? json = null;

        // Act
        bool result = EffectUtils.TryConvert(json!, out var effect);

        // Assert
        result.Should().BeFalse();
        effect.Should().BeNull();
    }
}