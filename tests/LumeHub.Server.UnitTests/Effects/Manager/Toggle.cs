using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Core.LedControl;
using LumeHub.Server.Effects;
using Microsoft.Extensions.Options;
using System.Text.Json;

namespace LumeHub.Server.UnitTests.Effects.Manager;

public sealed class Toggle
{
    [Fact]
    public void IsOnIsTrue_When_ToggleWithStateTrue()
    {
        // Arrange
        var ledController = Substitute.For<LedController>(Options.Create(new LedControllerOptions { PixelCount = 100 }));
        var effectQueueingService = Substitute.For<QueueingService>(ledController);
        var manager = new LumeHub.Server.Effects.Manager(effectQueueingService);
        var effect = new FadeColor { Color = new RgbColor(123, 45, 6) };
        var effectDto = new EffectDto
        {
            Id = Guid.NewGuid().ToString(),
            Name = "Name",
            Data = JsonSerializer.Serialize(effect)
        };

        // Act
        manager.SetEffect(effectDto);
        manager.Toggle(true);

        // Assert
        manager.IsOn.Should().BeTrue();
    }
}