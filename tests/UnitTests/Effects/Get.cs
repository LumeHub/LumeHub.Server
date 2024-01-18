using LumeHub.Api.Effects;
using LumeHub.Api.Effects.Get;

namespace UnitTests.Effects;

public sealed class Get
{
    [Fact]
    public async void ReturnsNotFound_When_IdDoesNotExit()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(false);

        var endpoint = Factory.Create<Endpoint>(repository);
        endpoint.Map = new Mapper();

        var request = new Request { Id = id };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NotFound);
    }

    [Fact]
    public async void ReturnsCorrectEffect_When_EffectWithProvidedIdExists()
    {
        // Arrange
        var effect = new EffectDto { Id = Guid.NewGuid().ToString(), Name = "Name", Data = "Data" };
        var repository = Substitute.For<IRepository>();
        repository.Exists(effect.Id).Returns(true);
        repository.Get(effect.Id).Returns(effect);

        var endpoint = Factory.Create<Endpoint>(repository);
        endpoint.Map = new Mapper();

        var request = new Request { Id = effect.Id };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.OK);
        endpoint.Response.Should().BeEquivalentTo(effect);
    }
}
