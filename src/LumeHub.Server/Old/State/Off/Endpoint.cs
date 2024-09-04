using LumeHub.Server.Effects;

namespace LumeHub.Server.Old.State.Off;

class Endpoint(IManager effectManager) : EndpointWithoutRequest<Dto>
{
    public override void Configure()
    {
        Post("led/state/off");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        if (effectManager.IsOn) effectManager.Toggle(false);

        await SendOkAsync(new Dto
        {
            LedState = effectManager.IsOn,
        }, ct);
    }
}