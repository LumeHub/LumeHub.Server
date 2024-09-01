using LumeHub.Core.Colors;
using LumeHub.Core.Effects.Normal;
using LumeHub.Server.Effects;
using System.Text.Json;

namespace LumeHub.Server.Old.Color.Set;

class Endpoint(IManager effectManager) : Endpoint<RgbColor>
{
    public override void Configure()
    {
        Post("led/setColor");
        AllowAnonymous();
    }

    public override async Task HandleAsync(RgbColor req, CancellationToken ct)
    {
        var effect = new FadeColor
        {
            Color = req,
        };

        effectManager.SetEffect(new EffectDto
        {
            Id = Guid.NewGuid().ToString(),
            Name = "Fade Color",
            Data = JsonSerializer.Serialize(effect),
        });

        await SendOkAsync(req, ct);
    }
}