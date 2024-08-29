using System;

static class Badge
{

    const string DEFAULT_DEPARTMENT = "Owner";
    public static string Print(int? id, string name, string? department)
    {

        if (id != null && department != null)
        {
            return $"[{id}] - {name} - {department.ToUpper()}";
        }


        if (id != null)
        {
            return $"[{id}] - {name} - {DEFAULT_DEPARTMENT.ToUpper()}";
        }

        if (department != null)
        {
            return $"{name} - {department.ToUpper()}";
        }

        return $"{name} - {DEFAULT_DEPARTMENT.ToUpper()}";
    }
}
