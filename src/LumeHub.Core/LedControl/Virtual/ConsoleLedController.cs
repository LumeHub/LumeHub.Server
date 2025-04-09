using LumeHub.Core.Colors;
using Microsoft.Extensions.Options;

namespace LumeHub.Core.LedControl.Virtual;

public sealed class VirtualLedController(IOptions<LedControllerOptions> options) : LedController(options)
{
    private readonly RgbColor[] _leds = new RgbColor[options.Value.PixelCount];

    public override RgbColor this[int index]
    {
        get => _leds[index];
        set => _leds[index] = value;
    }

    public override void Show()
    {
        foreach (var color in _leds)
        {
            Console.Write($"\x1b[38;2;{color.Red};{color.Green};{color.Blue}m" + // Foreground
                          $"\x1b[48;2;{color.Red};{color.Green};{color.Blue}m"); // Background
            Console.Write(' ');                                                  // Write any char
            Console.Write("\x1b[0m");                                            // Reset the colors
        }

        Console.WriteLine();
    }
} 