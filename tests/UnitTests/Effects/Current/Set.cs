using LumeHub.Server.Effects;
using LumeHub.Server.Effects.Current.Set;

namespace UnitTests.Effects.Current;
public sealed class Set
{
    [Fact]
    public async void ReturnsNoContent_When_EffectWithIdExists()
    {
        // Arrange
        string id = Guid.NewGuid().ToString();
        var effectDto = new EffectDto
        {
            Id = id,
            Name = "Name",
            Data = "Data"
        };
        var repository = Substitute.For<IRepository>();
        repository.Exists(id).Returns(true);
        repository.Get(id).Returns(effectDto);

        var manager = Substitute.For<IManager>();
        var endpoint = Factory.Create<Endpoint>(repository, manager);
        var request = new Request { Id = id };

        // Act
        await endpoint.HandleAsync(request, default);
        int statusCode = endpoint.HttpContext.Response.StatusCode;

        // Assert
        statusCode.Should().Be((int)HttpStatusCode.NoContent);
        manager.Received().SetEffect(Arg.Is<EffectDto>(e => e == effectDto));
    }
}