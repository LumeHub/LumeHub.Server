namespace LumeHub.Server.Effects.Update;

public sealed class Request
{
    public required string Id { get; init; }
    public required string Name { get; init; }
    public required string Data { get; init; }
}