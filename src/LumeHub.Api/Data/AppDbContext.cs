using LumeHub.Api.Effects;
using Microsoft.EntityFrameworkCore;

namespace LumeHub.Api.Data;

public sealed class AppDbContext(DbContextOptions<AppDbContext> options) : DbContext(options)
{
    public required DbSet<EffectDto> Effects { get; init; }
}
