static class AssemblyLine
{
    static readonly int CARS_PER_HOUR = 221;

    public static double SuccessRate(int speed)
    {
        if (speed == 0)
        {
            return 0;
        }
        else if (speed < 5)
        {
            return 1;
        }
        else if (speed < 9)
        {
            return 0.9;
        }
        else if (speed < 10)
        {
            return 0.8;
        }
        return 0.77;
    }

    public static double ProductionRatePerHour(int speed)
    {
        return speed * CARS_PER_HOUR * SuccessRate(speed);
    }

    public static int WorkingItemsPerMinute(int speed)
    {
        return (int)ProductionRatePerHour(speed) / 60;
    }
}
