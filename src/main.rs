mod math_loc;
// what the fuck have I done ...
use crate::math_loc::ranges::F64Range;

fn main() {
    let test1 = F64Range::calc_steps_rounded(-0.3, 1.0, 0.1);
    print!("Test 1: {}\n", test1);

    for i in F64Range::new(-0.3, 0.7, 0.1) {
        print!("Test 2: {}\n", i);
    }

    return;
}
