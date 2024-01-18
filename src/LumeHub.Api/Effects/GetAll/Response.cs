namespace LumeHub.Api.Effects.GetAll;

public sealed class Response
{
    public sealed class Effect
    {
        public required string Id { get; init; }
        public required string Name { get; init; }
        public required string Data { get; init; }
    }

    public required IEnumerable<Effect> Effects { get; init; }
}