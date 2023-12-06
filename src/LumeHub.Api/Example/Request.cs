namespace LumeHub.Api.Example;

public sealed class Request
{
    [QueryParam]
    public string? String { get; set; }
}