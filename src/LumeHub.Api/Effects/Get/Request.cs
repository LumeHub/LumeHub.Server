namespace LumeHub.Api.Effects.Get;

public sealed class Request
{
    [QueryParam]
    public required string Id { get; init; }
}