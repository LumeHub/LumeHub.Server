using LumeHub.Core.Colors;
using LumeHub.Core.Effects;
using LumeHub.Core.Effects.Normal;
using System.Text.Json;

namespace LumeHub.Core.UnitTests.Effects;

public sealed class EffectConverterFacts
{
    [Fact]
    public void Write_Should_SerializeEffectWithTypeProperty()
    {
        // Arrange
        var effect = new FadeColor { Color = new RgbColor(255, 0, 0) };

        // Act
        string json = JsonSerializer.Serialize(effect);

        // Assert
        json.Should().Contain($"\"{nameof(Effect.Name)}\":\"FadeColor");
    }

    [Fact]
    public void Read_Should_DeserializeEffectBasedOnNameProperty()
    {
        // Arrange
        const string json = """{"Name":"FadeColor","Color":{"Red":255,"Green":0,"Blue":0}}""";

        // Act
        var deserializedEffect = JsonSerializer.Deserialize<Effect>(json);

        // Assert
        deserializedEffect.Should().NotBeNull();
        deserializedEffect.Should().BeOfType<FadeColor>();
        deserializedEffect!.Name.Should().Be("FadeColor");
    }

    [Fact]
    public void Read_Should_ReturnNull_WhenNamePropertyIsMissing()
    {
        // Arrange
        const string json = """{"Color":{"Red":255,"Green":0,"Blue":0}}""";

        // Act
        var deserializedEffect = JsonSerializer.Deserialize<Effect>(json);

        // Assert
        deserializedEffect.Should().BeNull();
    }

    [Fact]
    public void Read_Should_ReturnNull_WhenNamePropertyIsNotAString()
    {
        // Arrange
        const string json = """{"Name":42,"Color":{"Red":255,"Green":0,"Blue":0}}""";

        // Act
        var deserializedEffect = JsonSerializer.Deserialize<Effect>(json);

        // Assert
        deserializedEffect.Should().BeNull();
    }
}