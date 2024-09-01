using LumeHub.Core.Colors;
using LumeHub.Core.Effects;
using LumeHub.Core.Effects.Normal;
using LumeHub.Server.Effects;

namespace LumeHub.Server.Old.Color.Get;

class Endpoint(IManager effectManager) : EndpointWithoutRequest<RgbColor>
{
    public override void Configure()
    {
        Post("led/getColor");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken cancellationToken = default)
    {
        if (effectManager.CurrentEffect is null
                || !EffectUtils.TryConvert(effectManager.CurrentEffect.Data, out var effect))
        {
            await SendAsync(new RgbColor());
            return;
        }

        await SendAsync(((FadeColor)effect!).Color);
    }
}