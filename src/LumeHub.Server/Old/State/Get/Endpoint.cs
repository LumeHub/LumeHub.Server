using LumeHub.Server.Effects;

namespace LumeHub.Server.Old.State.Get;

class Endpoint(IManager effectManager) : EndpointWithoutRequest<Dto>
{
    public override void Configure()
    {
        Post("led/state/get");
        AllowAnonymous();
    }

    public override async Task HandleAsync(CancellationToken ct)
    {
        await SendOkAsync(new Dto
        {
            LedState = effectManager.IsOn,
        }, ct);
    }
}