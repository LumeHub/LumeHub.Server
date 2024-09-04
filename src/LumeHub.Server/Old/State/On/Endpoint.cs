using LumeHub.Server.Effects;

namespace LumeHub.Server.Old.State.On;

class Endpoint(IManager effectManager) : EndpointWithoutRequest<Dto>
{
    public override void Configure()
    {
        Post("led/state/on");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        if (!effectManager.IsOn) effectManager.Toggle(true);

        await SendOkAsync(new Dto
        {
            LedState = effectManager.IsOn,
        }, ct);
    }
}