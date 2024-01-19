using FastEndpoints.Swagger;
using ApiLocksmith.Core.Extensions;
using ApiLocksmith.Swagger.FastEndpoints.Extensions;
using LumeHub.Core.LedControl;
using Effects = LumeHub.Api.Effects;

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

var app = builder.Build();

// Configure the HTTP request pipeline.

app.UseAuthentication()
    .UseAuthorization()
    .UseFastEndpoints();

if (app.Environment.IsDevelopment())
{
    app.UseSwaggerGen();
}

app.Run();
