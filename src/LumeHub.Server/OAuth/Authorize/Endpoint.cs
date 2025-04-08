namespace LumeHub.Server.OAuth;

public class AuthorizeEndpoint : EndpointWithoutRequest
{
    public override void Configure()
    {
        Get("oauth/authorize");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        // parse the redirect params
        var redirectUri = HttpContext.Request.Query["redirect_uri"].ToString();
        var state = HttpContext.Request.Query["state"].ToString();
        var clientId = HttpContext.Request.Query["client_id"].ToString();

        // simulate login + redirect
        var code = "mock-auth-code"; // any string is okay
        var uri = $"{redirectUri}?code={code}&state={state}";

        await SendRedirectAsync(uri, permanent: false, cancellation: ct);
    }
}
