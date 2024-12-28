using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Server.Effects;
using LumeHub.Server.Effects.Update;
using System.Text.Json;

namespace LumeHub.Server.UnitTests.Effects;

public sealed class Update
{
    [Fact]
    public async void ReturnsNotFound_When_EffectDoesNotExist()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(false);

        var endpoint = Factory.Create<Endpoint>(repository);
        var request = new Request
        {
            Id = id,
            Name = "New Name",
            Data = "New Data"
        };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NotFound);
    }

    [Fact]
    public async void Throws_When_DataIsInvalid()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(true);

        var endpoint = Factory.Create<Endpoint>(repository);

        var request = new Request { Id = id, Name = "New Name", Data = "Invalid new data" };

        // Act
        var func = () => endpoint.HandleAsync(request, default);

        // Assert
        await func.Should().ThrowAsync<Exception>();
    }

    [Fact]
    public async void ReturnsNoContent_When_RequestIsValid()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(true);
        repository.Get(id).Returns(new EffectDto { Id = id, Name = "Original Name", Data = "Data" });

        var endpoint = Factory.Create<Endpoint>(repository);
        var request = new Request
        {
            Id = id,
            Name = "New Name",
            Data = JsonSerializer.Serialize(new FadeColor { Color = new RgbColor(123, 45, 6) })
        };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NoContent);
        repository.Received().Update(Arg.Is<EffectDto>(e => 
            e.Name == request.Name &&
            e.Data == request.Data));
    }
}