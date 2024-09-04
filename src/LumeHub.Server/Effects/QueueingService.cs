using LumeHub.Core.Effects;
using LumeHub.Core.LedControl;
using System.Collections.Concurrent;

namespace LumeHub.Server.Effects;

public class QueueingService(LedController ledController) : BackgroundService
{
    private readonly ConcurrentQueue<Effect> _effectQueue = new();
    private readonly SemaphoreSlim _semaphore = new(1, 1);

    public void Enqueue(Effect effect)
    {
        _effectQueue.Clear();
        _effectQueue.Enqueue(effect);
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        await Task.Run(async () =>
        {
            while (!stoppingToken.IsCancellationRequested)
            {
                await DoWork(stoppingToken);
            }
        });
    }

    private async Task DoWork(CancellationToken stoppingToken)
    {
        if (!_effectQueue.TryDequeue(out var effect)) return;

        await _semaphore.WaitAsync(stoppingToken);
        try
        {
            effect.Apply(ledController);
        }
        finally
        {
            _semaphore.Release();
        }
    }

    public override async Task StopAsync(CancellationToken stoppingToken)
        => await _semaphore.WaitAsync(stoppingToken);
}