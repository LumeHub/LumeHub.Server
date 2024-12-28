using LumeHub.Server.Effects;
using LumeHub.Server.Effects.Current.Get;

namespace LumeHub.Server.UnitTests.Effects.Current;

public sealed class Get
{
    [Fact]
    public async void ReturnsNoContent_WhenCurrentEffectIsNull()
    {
        // Arrange
        var effectManager = Substitute.For<IManager>();
        effectManager.CurrentEffect.Returns((EffectDto?)null);

        var endpoint = Factory.Create<Endpoint>(effectManager);
        endpoint.Map = new Mapper();

        // Act
        await endpoint.HandleAsync(default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NoContent);
    }

    [Fact]
    public async void ReturnsCorrectEffect_When_EffectWithProvidedIdExists()
    {
        // Arrange
        var effect = new EffectDto { Id = Guid.NewGuid().ToString(), Name = "Name", Data = "Data" };
        var effectManager = Substitute.For<IManager>();
        effectManager.CurrentEffect.Returns(effect);

        var endpoint = Factory.Create<Endpoint>(effectManager);
        endpoint.Map = new Mapper();

        // Act
        await endpoint.HandleAsync(default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.OK);
        endpoint.Response.Should().BeEquivalentTo(effect, o => o.Excluding(e => e.Effect));
    }
}