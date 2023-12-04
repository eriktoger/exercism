pub fn isPangram(str: []const u8) bool {
    var alphabet = [_]bool{false} ** 26;

    for (str) |chr| {
        if (chr >= 65 and chr <= 90) {
            const index = chr - 65;
            alphabet[index] = true;
        }
        if (chr >= 97 and chr <= 122) {
            const index = chr - 65 - 32;
            alphabet[index] = true;
        }
    }

    for (alphabet) |found| {
        if (!found) {
            return false;
        }
    }

    return true;
}
