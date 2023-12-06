using LumeHub.Api.Example;

namespace UnitTests.Example;

public sealed class EndpointFacts
{
    [Fact]
    public async void ReturnsNull()
    {
        // Arrange
        var endpoint = Factory.Create<Endpoint>();
        endpoint.Map = new Mapper();
        var req = new Request();

        // Act
        await endpoint.HandleAsync(req, default);
        var response = endpoint.Response;

        // Assert
        response.String.Should().BeNull();
    }

    [Fact]
    public async void ReturnsValueOfRequest()
    {
        // Arrange
        var endpoint = Factory.Create<Endpoint>();
        endpoint.Map = new Mapper();
        var req = new Request
        {
            String = "test"
        };

        // Act
        await endpoint.HandleAsync(req, default);
        var response = endpoint.Response;

        // Assert
        response.String.Should().BeEquivalentTo(req.String);
    }
}
