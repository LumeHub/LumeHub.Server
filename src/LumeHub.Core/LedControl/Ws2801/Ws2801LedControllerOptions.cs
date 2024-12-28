namespace LumeHub.Core.LedControl.Ws2801;

public class Ws2801LedControllerOptions : LedControllerOptions
{
    public required int BusId { get; init; } = 0;
    public required int ClockFrequency { get; init; } = 500_000;
}