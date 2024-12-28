﻿using LumeHub.Server.Effects;
using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using System.Text.Json;
using LumeHub.Core.LedControl;
using Microsoft.Extensions.Logging;
using Microsoft.Extensions.Options;

namespace UnitTests.Effects.Manager;

public sealed class SetEffect
{
    [Fact]
    public void CurrentEffectGetsUpdate_When_FirstEffectIsApplied()
    {
        // Arrange
        var ledController = Substitute.For<LedController>(Options.Create(new LedControllerOptions { PixelCount = 100 }));
        var effectQueueingService = Substitute.For<QueueingService>(ledController);
        var manager = new LumeHub.Server.Effects.Manager(effectQueueingService);
        var effect = new EffectDto
        {
            Id = Guid.NewGuid().ToString(),
            Name = "Test",
            Data = JsonSerializer.Serialize(new FadeColor { Color = new RgbColor(123, 45, 6) })
        };

        // Act
        manager.SetEffect(effect);

        // Assert
        manager.CurrentEffect.Should().BeEquivalentTo(effect);
    }
}