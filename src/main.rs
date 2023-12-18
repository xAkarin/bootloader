mod mathlib;

// what the fuck have I done ...
use crate::mathlib::floatingrange::FloatingPointRange;  

fn main() {
    let test1 = FloatingPointRange::calc_steps_rounded(-0.3, 1.0, 0.1);

    print!("Test 1: {}\n", test1);

    for i in FloatingPointRange::new(-0.3, 0.7, 0.1) {
        print!("{}\n", i);
    }

    return;
}
