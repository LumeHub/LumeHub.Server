namespace LumeHub.Server.SmartHome;

public sealed class Response
{
    public required string RequestId { get; init; }
    public required object Payload { get; init; }
}
