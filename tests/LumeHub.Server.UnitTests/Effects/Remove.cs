using LumeHub.Server.Effects;
using LumeHub.Server.Effects.Remove;

namespace LumeHub.Server.UnitTests.Effects;

public sealed class Remove
{
    [Fact]
    public async void ReturnsNotFound_When_EffectDoesNotExist()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(false);

        var endpoint = Factory.Create<Endpoint>(repository);
        var request = new Request { Id = id };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NotFound);
        repository.DidNotReceive().Remove(Arg.Any<string>());
    }

    [Fact]
    public async void ReturnsNoContent_When_EffectExists()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(true);

        var endpoint = Factory.Create<Endpoint>(repository);
        var request = new Request { Id = id };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NoContent);
        repository.Received().Remove(Arg.Is<string>(p => p == id));
    }
}