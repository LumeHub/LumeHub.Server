namespace LumeHub.Server.SmartHome;

public sealed class Request
{
    public required string RequestId { get; init; }
    public required List<SmartHomeInput>? Inputs { get; init; }
}

public sealed class SmartHomeInput
{
    public required string Intent { get; init; }
    public SmartHomePayload? Payload { get; init; }
}

public sealed class SmartHomePayload
{
    public List<SmartHomeCommand>? Commands { get; init; }
}

public sealed class SmartHomeCommand
{
    public List<Dictionary<string, string>>? Devices { get; init; }
    public List<SmartHomeExecution>? Execution { get; init; }
}

public sealed class SmartHomeExecution
{
    public required string Command { get; init; }
    public SmartHomeExecutionParams? Params { get; init; }
}

public sealed class SmartHomeExecutionParams
{
    public bool On { get; init; }
}
