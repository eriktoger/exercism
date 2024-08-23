class RemoteControlCar(int speed, int batteryDrain)
{
    private readonly int Speed = speed;
    private readonly int BatteryDrain = batteryDrain;
    private int Battery = 100;
    private int Distance = 0;

    public bool BatteryDrained()
    {
        return Battery < BatteryDrain;
    }

    public int DistanceDriven()
    {
        return Distance;
    }

    public void Drive()
    {
        if (!BatteryDrained())
        {
            Distance += Speed;
            Battery -= BatteryDrain;
        }
    }

    public static RemoteControlCar Nitro()
    {
        return new RemoteControlCar(50, 4);
    }
}

class RaceTrack(int distance)
{
    private readonly int Distance = distance;
    public bool TryFinishTrack(RemoteControlCar car)
    {
        while (!car.BatteryDrained())
        {
            car.Drive();
            if (car.DistanceDriven() >= Distance)
            {
                return true;
            }
        }

        return false;
    }
}
