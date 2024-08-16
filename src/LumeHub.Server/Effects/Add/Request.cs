﻿namespace LumeHub.Server.Effects.Add;

public sealed class Request
{
    public required string Name { get; init; }
    public required string Data { get; init; }
}