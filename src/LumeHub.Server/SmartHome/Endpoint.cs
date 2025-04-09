using LumeHub.Server.Effects;

namespace LumeHub.Server.SmartHome;

public sealed class Endpoint(IManager effectManager) : Endpoint<Request, Response>
{
    private const string Sync = "action.devices.SYNC";
    private const string Query = "action.devices.QUERY";
    private const string Execute = "action.devices.EXECUTE";
    private const string OnOff = "action.devices.commands.OnOff";
    private const string DeviceId = "led-strip";
    
    public override void Configure()
    {
        Post("smarthome");
        AllowAnonymous();
    }

    public override async Task HandleAsync(Request req, CancellationToken ct)
    {
        string? intent = req.Inputs?.FirstOrDefault()?.Intent;
        string requestId = req.RequestId;

        if (string.IsNullOrWhiteSpace(intent))
        {
            await SendAsync(new Response
            {
                RequestId = requestId,
                Payload = new { error = "Missing intent" }
            }, cancellation: ct);
            return;
        }

        var response = intent switch
        {
            Sync => HandleSyncIntent(requestId),
            Query => HandleQueryIntent(requestId),
            Execute => HandleExecuteIntent(req, requestId),
            _ => new Response { RequestId = requestId, Payload = new { error = "Unsupported intent" } }
        };

        await SendAsync(response, cancellation: ct);
    }
    
    private static Response HandleSyncIntent(string requestId) => new()
    {
        RequestId = requestId,
        Payload = new
        {
            agentUserId = "123",
            devices = new[]
            {
                new
                {
                    id = DeviceId,
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
    };

    private Response HandleQueryIntent(string requestId) => new()
    {
        RequestId = requestId,
        Payload = new
        {
            devices = new Dictionary<string, object>
            {
                [DeviceId] = new { on = effectManager.IsOn, online = true }
            }
        }
    };

    private Response HandleExecuteIntent(Request req, string requestId)
    {
        var execution = req.Inputs?
            .FirstOrDefault()?.Payload?
            .Commands?.FirstOrDefault()?
            .Execution?.FirstOrDefault();

        if (execution?.Command != OnOff)
        {
            return new Response
            {
                RequestId = requestId,
                Payload = new
                {
                    commands = new[]
                    {
                        new { ids = new[] { DeviceId }, status = "ERROR", errorCode = "unsupportedCommand" }
                    }
                }
            };
        }

        bool turnOn = execution.Params?.On ?? false;
        effectManager.Toggle(turnOn);

        return new Response
        {
            RequestId = requestId,
            Payload = new
            {
                commands = new[]
                {
                    new
                    {
                        ids = new[] { DeviceId },
                        status = "SUCCESS",
                        states = new { on = turnOn, online = true }
                    }
                }
            }
        };

    }
}