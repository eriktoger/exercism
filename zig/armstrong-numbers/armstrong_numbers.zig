const math = @import("std").math;

pub fn isArmstrongNumber(num: u128) bool {
    if (num == 0) {
        return true;
    }

    var length = math.log10(num) + 1;
    var sum: u128 = 0;
    var number = num;
    while (number > 0) {
        var digit = number % 10;
        sum += math.pow(u128, digit, length);
        number /= 10;
    }

    return sum == num;
}
