namespace LumeHub.Server.OAuth.Token;

public sealed class Request
{
    [BindFrom("grant_type")]
    public required string GrantType { get; init; }

    [BindFrom("client_id")]
    public required string ClientId { get; init; }
    
    [BindFrom("refresh_token")]
    public required string RefreshToken { get; init; }
}