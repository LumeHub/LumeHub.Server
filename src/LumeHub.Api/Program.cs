using FastEndpoints.Swagger;
using ApiLocksmith.Core.Extensions;
using ApiLocksmith.Swagger.FastEndpoints.Extensions;

var builder = WebApplication.CreateBuilder(args);
const string authSectionKey = "ApiKeySettings";

// Add services to the container.

builder.Services
    .AddAuthorization()
    .AddFastEndpoints()
    .AddApiKeyAuthenticationScheme(builder.Configuration, authSectionKey)
    .AddSwaggerDocumentWithApiKeyAuth(builder.Configuration, authSectionKey);

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
