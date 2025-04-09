namespace LumeHub.Server.OAuth.Authorize;

public class Request
{
    [BindFrom("redirect_uri")]
    public required string RedirectUri { get; init; }
    [BindFrom("state")]
    public required string State { get; init; }
}