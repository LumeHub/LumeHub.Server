using LumeHub.Server.Effects;

namespace LumeHub.Server.SmartHome;

public sealed class Endpoint(IManager effectManager) : Endpoint<SmartHomeRequest, SmartHomeResponse>
{
    public override void Configure()
    {
        Post("smarthome");
        AllowAnonymous();
    }

    public override async Task HandleAsync(SmartHomeRequest req, CancellationToken ct)
    {
        var intent = req.Inputs?.FirstOrDefault()?.Intent;
        var requestId = req.RequestId;

        SmartHomeResponse response = intent switch
        {
            "action.devices.SYNC" => new()
            {
                RequestId = requestId,
                Payload = new
                {
                    agentUserId = "123",
                    devices = new[]
                    {
                        new
                        {
                            id = "led-strip",
                            type = "action.devices.types.LIGHT",
                            traits = new[] { "action.devices.traits.OnOff" },
                            name = new { name = "LED Strip" },
                            willReportState = false,
                            deviceInfo = new
                            {
                                manufacturer = "LumeHub",
                                model = "LEDv1"
                            }
                        }
                    }
                }
            },

            "action.devices.QUERY" => new()
            {
                RequestId = requestId,
                Payload = new
                {
                    devices = new Dictionary<string, object>
                    {
                        { "led-strip", new { on = effectManager.IsOn, online = true } }
                    }
                }
            },

            "action.devices.EXECUTE" => await HandleExecuteIntent(req, requestId),

            _ => new() { RequestId = requestId, Payload = new { } }
        };

        await SendAsync(response, cancellation: ct);
    }

    private static SmartHomeResponse HandleExecuteIntent(SmartHomeRequest req, string requestId)
    {
        var command = req.Inputs?.FirstOrDefault()?.Payload?.Commands?.FirstOrDefault();
        var execution = command?.Execution?.FirstOrDefault();

        if (execution?.Command == "action.devices.commands.OnOff")
        {
            var turnOn = execution.Params?.On ?? false;
            effectManager.Toggle(turnOn);

            return new SmartHomeResponse
            {
                RequestId = requestId,
                Payload = new
                {
                    commands = new[]
                    {
                        new
                        {
                            ids = new[] { "led-strip" },
                            status = "SUCCESS",
                            states = new { on = turnOn, online = true }
                        }
                    }
                }
            };
        }

        return new SmartHomeResponse
        {
            RequestId = requestId,
            Payload = new
            {
                commands = new[]
                {
                    new
                    {
                        ids = new[] { "led-strip" },
                        status = "ERROR",
                        errorCode = "unsupportedCommand"
                    }
                }
            }
        };
    }
}