using System;
using System.Linq;

public static class Identifier
{
    private static bool IsGreekLowercase(char chr)
    {
        return (chr >= '\u03B1' && chr <= '\u03C9') || chr == '\u03C2';
    }
    private static bool ForbiddenChars(char chr)
    {
        return IsGreekLowercase(chr);
    }
    private static bool AllowedChars(char chr)
    {
        return Char.IsLetter(chr) || chr == '_' || chr == '\0';
    }


    public static string Clean(string identifier)
    {
        var chrs = identifier.ToArray();
        for (int i = 0; i < (chrs.Length - 1); i++)
        {
            if (chrs[i] == '-')
            {
                chrs[i + 1] = Char.ToUpper(chrs[i + 1]);
            }
            if (chrs[i] == ' ')
            {
                chrs[i] = '_';
            }
        }

        var filtered = chrs.Where(chr =>
                                  AllowedChars(chr)
                                  && !ForbiddenChars(chr))
                                  .ToArray();

        string str = new(filtered);
        return str.Replace("\0", "CTRL");
    }
}
