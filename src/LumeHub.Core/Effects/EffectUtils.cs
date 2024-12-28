using System.Text.Json;

namespace LumeHub.Core.Effects;

/// <summary>
/// Utility class for working with effects.
/// </summary>
public static class EffectUtils
{
    /// <summary>
    /// Tries to convert the given JSON string to an Effect object.
    /// </summary>
    /// <param name="json">The JSON string representing the Effect object.</param>
    /// <param name="effect">When the method returns, contains the converted Effect object if successful; otherwise, <c>null</c>.</param>
    /// <returns>
    /// <c>true</c> if the JSON string was successfully converted to an Effect object; otherwise, <c>false</c>.
    /// </returns>
    public static bool TryConvert(string json, out Effect? effect)
    {
        effect = null;
        try
        {
            var deserializedEffect = JsonSerializer.Deserialize<Effect>(json);

            if (deserializedEffect is null)
                return false;

            // Check if any properties of the deserialized object are null
            var type = deserializedEffect.GetType();
            if (type.GetProperties().Any(p => p.GetValue(deserializedEffect) is null))
            {
                return false;
            }

            effect = deserializedEffect;
            return true;
        }
        catch (Exception)
        {
            // An exception occurred during deserialization, set effect to null and return false
            return false;
        }
    }
}
