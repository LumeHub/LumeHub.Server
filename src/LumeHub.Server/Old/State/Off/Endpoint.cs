using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Core.LedControl;
using LumeHub.Server.Effects;

namespace LumeHub.Server.Old.State.Off;

class Endpoint(IManager effectManager, LedController ledController) : EndpointWithoutRequest<Dto>
{
    public override void Configure()
    {
        Post("led/state/off");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        if (effectManager.IsOn)
        {
            var effect = new FadeColor { Color = new RgbColor() };
            effect.Apply(ledController);

            effectManager.Toggle(false);
        }

        await SendOkAsync(new Dto
        {
            LedState = effectManager.IsOn,
        }, ct);
    }
}