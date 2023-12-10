pub fn isIsogram(str: []const u8) bool {
    var alphabet = [_]usize{0} ** 26;
    for (str) |chr| {
        if (chr >= 65 and chr <= 90) {
            const index = chr - 65;
            alphabet[index] += 1;
        } else if (chr >= 97 and chr <= 122) {
            const index = chr - 65 - 32;
            alphabet[index] += 1;
        }
    }

    for (alphabet) |count| {
        if (count > 1) {
            return false;
        }
    }
    return true;
}
