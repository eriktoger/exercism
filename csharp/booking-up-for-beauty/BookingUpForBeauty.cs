using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.CompilerServices;

static class Appointment
{
    private static readonly List<string> Months = ["January", "Feburary", "March",
                                                   "April","May", "June",
                                                   "July","August", "September",
                                                   "October", "November", "December"];

    private static DateTime HandleOnlyNumbers(string appointmentDateDescription)
    {
        var dateTime = appointmentDateDescription.Split(" ");

        var date = dateTime[0].Split("/");
        var year = int.Parse(date[2]);
        var month = int.Parse(date[0]);
        var day = int.Parse(date[1]);

        var time = dateTime[1].Split(":");
        var hour = int.Parse(time[0]);
        var minute = int.Parse(time[1]);
        var second = int.Parse(time[2]);

        return new DateTime(year, month, day, hour, minute, second);
    }

    private static DateTime HandleTextual(string appointmentDateDescription)
    {
        var dateTime = appointmentDateDescription.Split(",");
        var dateTimeNoWeekday = dateTime.Length == 3 ? dateTime.ToList().Skip(1).ToArray() : dateTime;

        var monthAndDay = dateTimeNoWeekday[0].Trim().Split(" ");
        var month = Months.IndexOf(monthAndDay[0]) + 1;
        var day = int.Parse(monthAndDay[1]);

        var yearAndTime = dateTimeNoWeekday[1].Split(" ");
        var year = int.Parse(yearAndTime[1]);
        var time = yearAndTime[2].Split(":");
        var hour = int.Parse(time[0]);
        var minute = int.Parse(time[1]);
        var second = int.Parse(time[2]);

        return new DateTime(year, month, day, hour, minute, second);

    }
    public static DateTime Schedule(string appointmentDateDescription)
    {
        if (appointmentDateDescription.Contains('/'))
        {
            return HandleOnlyNumbers(appointmentDateDescription);
        }

        if (appointmentDateDescription.Contains(','))
        {
            return HandleTextual(appointmentDateDescription);
        }

        return new DateTime();
    }

    public static bool HasPassed(DateTime appointmentDate)
    {
        return DateTime.Now > appointmentDate;
    }

    public static bool IsAfternoonAppointment(DateTime appointmentDate)
    {
        var hour = appointmentDate.Hour;
        return hour > 11 && hour < 18;
    }

    public static string Description(DateTime appointmentDate)
    {
        var day = appointmentDate.Day;
        var month = appointmentDate.Month;
        var year = appointmentDate.Year;

        var hour = appointmentDate.Hour;
        var period = hour > 12 ? "PM" : "AM";
        var hourWithPeriod = hour > 12 ? hour - 12 : hour;
        var minute = appointmentDate.Minute < 10 ? $"0{appointmentDate.Minute}" : $"{appointmentDate.Minute}";
        var second = appointmentDate.Second < 10 ? $"0{appointmentDate.Second}" : $"{appointmentDate.Second}";

        return $"You have an appointment on {month}/{day}/{year} {hourWithPeriod}:{minute}:{second} {period}.";
    }

    public static DateTime AnniversaryDate()
    {
        return new DateTime(DateTime.Now.Year, 9, 15);
    }
}
