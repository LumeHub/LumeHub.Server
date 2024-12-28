using System.Drawing;

namespace LumeHub.Core.Colors;

/// <summary>
/// Provides extension methods for working with the <see cref="Color"/> class.
/// </summary>
public static class ColorExtensions
{
    /// <summary>
    /// Gets the name of the closest known color for a given color.
    /// </summary>
    public static string GetClosestKnownColorName(this Color color)
    {
        var closestColorDistance = new KeyValuePair<string, int>(Color.White.Name, int.MaxValue);

        foreach (var knownColor in Colors.KnownColorNameDictionary)
        {
            int distance = knownColor.Key.GetDistance(color);

            if (distance < closestColorDistance.Value)
            {
                closestColorDistance = new KeyValuePair<string, int>(knownColor.Value, distance);
            }
        }

        return closestColorDistance.Key;
    }

    /// <summary>
    /// Calculates the Euclidean distance between two colors.
    /// </summary>
    private static int GetDistance(this Color color1, Color color2)
    {
        int l = (color1.R - color2.R) * (color1.R - color2.R);
        int a = (color1.G - color2.G) * (color1.G - color2.G);
        int b = (color1.B - color2.B) * (color1.B - color2.B);

        return (int)Math.Sqrt(l + a + b);
    }
}