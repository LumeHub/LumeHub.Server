using Microsoft.AspNetCore.WebUtilities;

namespace LumeHub.Server.OAuth.Authorize;

public class Endpoint : Endpoint<Request>
{
    public override void Configure()
    {
        Get("oauth/authorize");
        AllowAnonymous();
    }

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        string uri = QueryHelpers.AddQueryString(req.RedirectUri, new Dictionary<string, string?>
        {
            ["code"] = Guid.NewGuid().ToString("N"),
            ["state"] = req.State
        });
        await SendRedirectAsync(uri, isPermanant: false, cancellation: ct);
    }
}