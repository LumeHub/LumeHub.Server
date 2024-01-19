using LumeHub.Core.Effects;

namespace LumeHub.Api.Effects;

public interface IRepository
{
    public string Add(EffectDto effect);
    public IEnumerable<EffectDto> GetAll();
    public EffectDto Get(string id);
    public void Remove(string id);
    bool Exists(string id);
    void Update(EffectDto effect);
}
public class Repository : IRepository
{
    public string Add(EffectDto effect) => throw new NotImplementedException();
    public IEnumerable<EffectDto> GetAll() => throw new NotImplementedException();
    public EffectDto Get(string id) => throw new NotImplementedException();
    public void Remove(string id) => throw new NotImplementedException();
    public bool Exists(string id) => throw new NotImplementedException();
    public void Update(EffectDto effect) => throw new NotImplementedException();
}
