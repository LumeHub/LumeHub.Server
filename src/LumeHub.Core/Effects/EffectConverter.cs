using LumeHub.Core.Effects.Normal;
using LumeHub.Core.Effects.Repeating;
using System.Text.Json;
using System.Text.Json.Serialization;

namespace LumeHub.Core.Effects;

public class EffectConverter : JsonConverter<Effect>
{
    public override Effect? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
        using var doc = JsonDocument.ParseValue(ref reader);
        var root = doc.RootElement;

        if (!root.TryGetProperty(nameof(Effect.Name), out var namePropertyValue)
            || namePropertyValue.ValueKind != JsonValueKind.String)
            return null;

        string? name = namePropertyValue.GetString();
        if (name is null) return null;

        var targetType = GetEffectType(name);
        if (targetType is null) return null;

        return (Effect?)JsonSerializer.Deserialize(root.GetRawText(), targetType, options);
    }

    private static Type? GetEffectType(string name) => name switch
    {
        // Normal
        nameof(FadeColor) => typeof(FadeColor),
        nameof(SetColor) => typeof(SetColor),
        // Repeating
        nameof(RainbowWave) => typeof(RainbowWave),
        _ => null
    };

    public override void Write(Utf8JsonWriter writer, Effect value, JsonSerializerOptions options) => JsonSerializer.Serialize(writer, value, value.GetType(), options);
}