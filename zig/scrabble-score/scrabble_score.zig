pub fn score(word: []const u8) u32 {
    var value: u32 = 0;

    for (word) |chr| {
        const uppercase = if (chr >= 97 and chr <= 122) chr - 32 else chr;
        value += switch (uppercase) {
            65, 69, 73, 79, 85, 76, 78, 82, 83, 84 => 1,
            68, 71 => 2,
            66, 67, 77, 80 => 3,
            70, 72, 86, 87, 89 => 4,
            75 => 5,
            74, 88 => 8,
            81, 90 => 10,
            else => 0,
        };
    }
    return value;
}
