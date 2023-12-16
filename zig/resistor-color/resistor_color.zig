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

pub fn colorCode(color: ColorBand) usize {
    return @intFromEnum(color);
}

const colorBand = [10]ColorBand{
    .black, .brown, .red,    .orange, .yellow,
    .green, .blue,  .violet, .grey,   .white,
};

pub fn colors() []const ColorBand {
    return &colorBand;
}
