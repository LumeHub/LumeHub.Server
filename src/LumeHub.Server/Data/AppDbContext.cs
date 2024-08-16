using LumeHub.Server.Effects;
using Microsoft.EntityFrameworkCore;

namespace LumeHub.Server.Data;

public sealed class AppDbContext(DbContextOptions<AppDbContext> options) : DbContext(options)
{
    public required DbSet<EffectDto> Effects { get; init; }
}