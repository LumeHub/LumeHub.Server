using FastEndpoints.Swagger;
using FastEndpoints.Security;

var builder = WebApplication.CreateBuilder(args);

// Add services to the container.

builder.Services
    .AddFastEndpoints()
    .SwaggerDocument()
    .AddJWTBearerAuth("Token")
    .AddAuthorization();

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
