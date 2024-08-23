using System;
using System.Runtime.CompilerServices;

static class LogLine
{
    public static string Message(string logLine)
    {
        var message = logLine.Split("]:");
        return message[1].Trim();
    }

    public static string LogLevel(string logLine)
    {
        var logLevel = logLine.Split("]:");
        return logLevel[0].Trim()[1..].ToLower();
    }

    public static string Reformat(string logLine)
    {
        var message = Message(logLine);
        var logLevel = LogLevel(logLine);

        return $"{message} ({logLevel})";
    }
}
