abstract class Character(string characterType)
{

    private readonly string CharacterType = characterType;
    protected bool IsVulnerable = false;

    public abstract int DamagePoints(Character target);

    public virtual bool Vulnerable()
    {
        return IsVulnerable;
    }

    public override string ToString()
    {
        return $"Character is a {CharacterType}";
    }
}

class Warrior : Character
{
    public Warrior() : base("Warrior")
    {
    }

    public override int DamagePoints(Character target)
    {
        return target.Vulnerable() ? 10 : 6;
    }
}

class Wizard : Character
{
    public Wizard() : base("Wizard")
    {
        IsVulnerable = true;
    }

    public override int DamagePoints(Character target)
    {
        return IsVulnerable ? 3 : 12;
    }

    public void PrepareSpell()
    {
        IsVulnerable = false;
    }
}
