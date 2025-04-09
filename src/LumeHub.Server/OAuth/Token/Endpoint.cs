namespace LumeHub.Server.OAuth.Token;

public class Endpoint : Endpoint<Request, Response>
{
    public override void Configure()
    {
        Post("oauth/token");
        AllowAnonymous();
        Description(x => x.Accepts<Request>("application/x-www-form-urlencoded"));
        Summary(s =>
        {
            s.Summary = "Google Smart Home OAuth 2.0 Token Endpoint";
            s.Description = "Exchanges authorization_code or refresh_token for access_token";
        });
    }

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        switch (req.GrantType)
        {
            case "authorization_code":
                await SendAsync(new Response
                {
                    AccessToken = "generated-access-token",
                    RefreshToken = "generated-refresh-token"
                }, cancellation: ct);
                break;
            case "refresh_token":
                await SendAsync(new Response
                {
                    AccessToken = "refreshed-access-token",
                    RefreshToken = req.RefreshToken
                }, cancellation: ct);
                break;
            default:
                await SendErrorsAsync(400, ct);
                break;
        }
    }
}
