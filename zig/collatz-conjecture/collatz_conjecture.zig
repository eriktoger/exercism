pub const ComputationError = error{
    IllegalArgument,
};

pub fn steps(number: usize) !usize {
    if (number == 0) {
        return ComputationError.IllegalArgument;
    }

    var num = number;
    var step: usize = 0;
    while (num != 1) {
        if (num % 2 == 0) {
            num /= 2;
            step += 1;
        } else {
            num = num * 3 + 1;
            step += 1;
        }
    }

    return step;
}
