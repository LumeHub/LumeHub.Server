using LumeHub.Server.Data;

namespace LumeHub.Server.Effects;

public interface IRepository
{
    public string Add(EffectDto effect);
    public IEnumerable<EffectDto> GetAll();
    public EffectDto Get(string id);
    public void Remove(string id);
    bool Exists(string id);
    void Update(EffectDto effect);
}
public class Repository(AppDbContext context) : IRepository
{
    public string Add(EffectDto effect)
    {
        context.Effects.Add(effect);
        context.SaveChanges();
        return effect.Id;
    }

    public IEnumerable<EffectDto> GetAll() => context.Effects;

    public EffectDto Get(string id) => context.Effects.Find(id)!;

    public void Remove(string id)
    {
        var effect = context.Effects.Find(id)!;
        context.Effects.Remove(effect);
        context.SaveChanges();
    }

    public bool Exists(string id) => context.Effects.Find(id) is not null;

    public void Update(EffectDto effect)
    {
        context.Effects.Update(effect);
        context.SaveChanges();
    }
}