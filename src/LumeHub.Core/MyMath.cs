namespace LumeHub.Core;

public static class MyMath
{
    public static T Max<T>(params T[] values) where T : IComparable<T>
    {
        if (values is null || values.Length == 0)
        {
            throw new ArgumentException("At least one value must be provided.");
        }

        var max = values[0];

        for (int i = 1; i < values.Length; i++)
            if (values[i].CompareTo(max) > 0) max = values[i];

        return max;
    }

    /// <summary>
    /// Really working implementation of the Modulus Operator when using negative numbers
    /// </summary>
    public static int Modulus(int a, int b) => a < 0 ? b + (a % b) : a % b;
}