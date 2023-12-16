const DASH = 45;
const X = 88;
const ZERO = 48;

fn count_dashes(s: []const u8) usize {
    var length = s.len;
    var dashes: usize = 0;

    while (length > 0) {
        length -= 1;
        const c = s[length];
        if (c == DASH) {
            dashes += 1;
        }
    }

    return dashes;
}

pub fn isValidIsbn10(s: []const u8) bool {
    const dashes = count_dashes(s);

    if (s.len - dashes != 10) {
        return false;
    }

    var sum: usize = 0;
    var multiplier: u8 = 10;

    for (s) |c| {
        if (c == DASH) {
            continue;
        }

        const isLastDigit = multiplier == 1;
        if (isLastDigit and c == X) {
            sum += 10;
        } else {
            sum += (c - ZERO) * multiplier;
        }

        multiplier -= 1;
    }

    return (sum % 11) == 0;
}
