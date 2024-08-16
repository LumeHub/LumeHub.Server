using FastEndpoints.Swagger;
using ApiLocksmith.Core.Extensions;
using ApiLocksmith.Swagger.FastEndpoints.Extensions;
using LumeHub.Core.LedControl;
using Microsoft.Data.Sqlite;
using Microsoft.EntityFrameworkCore;
using Effects = LumeHub.Server.Effects;
using LumeHub.Server.Data;

#if DEBUG
using LumeHub.Core.LedControl.Debug;
#else
using LumeHub.Core.LedControl.Ws2801;
#endif

var builder = WebApplication.CreateBuilder(args);
const string authSectionKey = "ApiKeySettings";

// Add services to the container.

builder.Services
    .AddAuthorization()
    .AddFastEndpoints()
    .AddApiKeyAuthenticationScheme(builder.Configuration, authSectionKey)
    .AddSwaggerDocumentWithApiKeyAuth(builder.Configuration, authSectionKey);

var ledControllerSection = builder.Configuration.GetSection("LedControllerSettings");
#if DEBUG
builder.Services
    .Configure<LedControllerOptions>(ledControllerSection)
    .AddSingleton<LedController, DebugLedController>();
#else
builder.Services
    .Configure<Ws2801LedControllerOptions>(ledControllerSection)
    .AddSingleton<LedController, Ws2801LedController>();
#endif

builder.Services
    .AddScoped<Effects.IRepository, Effects.Repository>()
    .AddSingleton<Effects.IManager, Effects.Manager>();

// DbContext
string folderPath = Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData);
builder.Services.AddDbContext<AppDbContext>(options =>
{
    options.UseSqlite(new SqliteConnectionStringBuilder
    {
        DataSource = Path.Combine(folderPath, builder.Configuration.GetConnectionString("DatabaseFileName")!)
    }.ToString());
    options.UseLoggerFactory(LoggerFactory.Create(loggerBuilder => loggerBuilder.AddFilter((category, level) =>
        category == DbLoggerCategory.Database.Command.Name
        && level == LogLevel.Warning)));
});

var app = builder.Build();

// Configure the HTTP request pipeline.

app.UseAuthentication()
    .UseAuthorization()
    .UseFastEndpoints();

// Migration
using var scope = app.Services.CreateScope();
var context = scope.ServiceProvider.GetRequiredService<AppDbContext>();
context.Database.Migrate();

if (app.Environment.IsDevelopment())
{
    app.UseSwaggerGen();
}

app.Run();