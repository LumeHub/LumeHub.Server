namespace LumeHub.Server.OAuth;

public class TokenRequest
{
    public string? Grant_Type { get; set; }
    public string? Code { get; set; }
    public string? Redirect_Uri { get; set; }
    public string? Client_Id { get; set; }
    public string? Client_Secret { get; set; }
}

public class TokenResponse
{
    public string Access_Token { get; set; } = "mock-access-token";
    public string Token_Type { get; set; } = "Bearer";
    public int Expires_In { get; set; } = 3600;
}

public class TokenEndpoint : Endpoint<TokenRequest, TokenResponse>
{
    public override void Configure()
    {
        Post("oauth/token");
        AllowAnonymous();
    }

    public override Task HandleAsync(TokenRequest r, CancellationToken ct)
    {
        return SendAsync(new TokenResponse(), cancellation: ct);
    }
}
