using LumeHub.Core.Effects;

namespace LumeHub.Api.Effects.Current.Update;

public sealed class Endpoint(IManager manager) : Endpoint<Request>
{
    public override void Configure() => Patch("effects/current");

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        if (!EffectUtils.TryConvert(req.Data, out var effect))
        {
            Logger.LogInformation("Could not convert data into a valid effect.");
            ThrowError(r => r.Data, "Could not convert data into a valid effect.");
            return;
        }

        manager.UpdateEffect(effect!);
        Logger.LogInformation("Updated the current effect.");
        await SendNoContentAsync(ct);
    }
}