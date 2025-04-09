namespace LumeHub.Server.OAuth.Token;

public sealed class Request
{
    [BindFrom("grant_type")]
    public required string GrantType { get; init; }

    [BindFrom("client_id")]
    public required string ClientId { get; init; }
    
    [BindFrom("refresh_token")]
    public string? RefreshToken { get; init; }

    public bool IsValid()
    {
        if (GrantType == "refresh_token" && string.IsNullOrEmpty(RefreshToken))
        {
            return false; // RefreshToken is required for the refresh_token grant type
        }
        return true;
    }
}