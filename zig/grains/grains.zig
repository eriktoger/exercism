pub const ChessboardError = error{IndexOutOfBounds};
const math = @import("std").math;

pub fn square(index: usize) ChessboardError!u64 {
    if (index == 0 or index > 64) {
        return ChessboardError.IndexOutOfBounds;
    }
    return math.pow(usize, 2, index - 1);
}

pub fn total() u64 {
    return @as(u64, @truncate(math.pow(u128, 2, 64) - 1));
}
