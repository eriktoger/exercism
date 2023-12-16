pub const ColorBand = enum {
    black,
    brown,
    red,
    orange,
    yellow,
    green,
    blue,
    violet,
    grey,
    white,
};

pub fn colorCode(colors: [2]ColorBand) usize {
    const first: usize = @intFromEnum(colors[0]);
    const second: usize = @intFromEnum(colors[1]);
    return first * 10 + second;
}
