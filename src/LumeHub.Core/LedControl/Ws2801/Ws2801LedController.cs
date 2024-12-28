using System.Device.Spi;
using Microsoft.Extensions.Options;

using LumeHub.Core.Colors;

namespace LumeHub.Core.LedControl.Ws2801;

public sealed class Ws2801LedController(IOptions<Ws2801LedControllerOptions> options) : LedController(options)
{
    private readonly SpiConnectionSettings _settings = new(options.Value.BusId)
    {
        ClockFrequency = options.Value.ClockFrequency
    };

    private readonly byte[] _buffer = new byte[options.Value.PixelCount * 3];

    public override RgbColor this[int index]
    {
        get
        {
            int offset = index * 3;
            return new RgbColor(_buffer[offset], _buffer[offset + 1], _buffer[offset + 2]);
        }
        set
        {
            int offset = index * 3;
            _buffer[offset + 0] = value.Red;
            _buffer[offset + 1] = value.Green;
            _buffer[offset + 2] = value.Blue;
        }
    }

    public override void Show()
    {
        using var device = SpiDevice.Create(_settings);
        device.Write(_buffer);
        Thread.Sleep(TimeSpan.FromMicroseconds(500));
    }
}