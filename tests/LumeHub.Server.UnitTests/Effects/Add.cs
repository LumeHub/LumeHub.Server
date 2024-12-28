using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Server.Effects;
using LumeHub.Server.Effects.Add;
using System.Text.Json;

namespace LumeHub.Server.UnitTests.Effects;

public sealed class Add
{
    [Fact]
    public async void ReturnsId_When_DataIsValid()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();

        var repository = Substitute.For<IRepository>();
        repository.Add(Arg.Any<EffectDto>()).Returns(id);

        var endpoint = Factory.Create<Endpoint>(repository);
        endpoint.Map = new Mapper();

        var req = new Request
        {
            Name = "EffectName",
            Data = JsonSerializer.Serialize(new FadeColor { Color = new RgbColor(255, 0, 255)})
        };

        // Act
        await endpoint.HandleAsync(req, default);
        var response = endpoint.Response;
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        response.Id.Should().Be(id);
        statusCode.Should().Be((int) HttpStatusCode.Created);
    }

    [Fact]
    public async void Throws_WhenDataIsInvalid()
    {
        // Arrange
        var repository = Substitute.For<IRepository>();
        var endpoint = Factory.Create<Endpoint>(repository);
        endpoint.Map = new Mapper();

        var req = new Request
        {
            Name = "EffectName",
            Data = "invalid json data"
        };

        // Act
        var func = () => endpoint.HandleAsync(req, default);

        // Assert
        await func.Should().ThrowAsync<Exception>();
    }
}