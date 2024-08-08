using System;

public class Player
{
    readonly Random random = new();
    public int RollDie()
    {
        return random.Next(1, 19);
    }

    public double GenerateSpellStrength()
    {
        return random.NextDouble() * 100;
    }
}
