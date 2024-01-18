namespace LumeHub.Api.Effects.Remove;

public sealed class Request
{
    [QueryParam]
    public required string Id { get; init; }
}