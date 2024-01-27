using LumeHub.Api.Effects;
using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Core.LedControl;
using Microsoft.Extensions.Options;
using System.Text.Json;

namespace UnitTests.Effects.Manager;

public sealed class UpdateEffect
{
    [Fact]
    public void CurrentEffectIsSet_When_NewEffectGetsApplied()
    {
        // Arrange
        var ledController = Substitute.For<LedController>(Options.Create(new LedControllerOptions { PixelCount = 100 }));
        ledController.SetAllPixel(new RgbColor());
        var manager = new LumeHub.Api.Effects.Manager(ledController);
        var effect = new FadeColor { Color = new RgbColor(123, 45, 6) };

        // Act
        manager.UpdateEffect(effect);

        // Assert
        manager.CurrentEffect.Should().BeEquivalentTo(new EffectDto
        {
            Id = "", Name = "", Data = JsonSerializer.Serialize(effect)
        });
    }

    [Fact]
    public void CurrentEffectIsUpdated_When_NewEffectGetsApplied()
    {
        // Arrange
        var ledController = Substitute.For<LedController>(Options.Create(new LedControllerOptions { PixelCount = 100 }));
        ledController.SetAllPixel(new RgbColor());
        var manager = new LumeHub.Api.Effects.Manager(ledController);
        var effect = new FadeColor { Color = new RgbColor(123, 45, 6) };
        var effectDto = new EffectDto
        {
            Id = Guid.NewGuid().ToString(), Name = "Name", Data = JsonSerializer.Serialize(effect)
        };

        // Act
        manager.SetEffect(effectDto);
        manager.UpdateEffect(effect);

        // Assert
        manager.CurrentEffect.Should().BeEquivalentTo(effectDto);
    }

}
