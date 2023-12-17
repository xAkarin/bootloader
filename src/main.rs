use std::{env, str};

fn main() {
    let args: Vec<String> = env::args().collect();

    // this barrows args
    dbg!("Args: {}\n", &args);

    let loc_int = Intergral {
        lower_limit: 1.0,
        higher_limit: 20.0,
        parsed_func: parse_args_to_func(&args),
    };

    print!("Calc\n");
    loc_int.calc();

    return;
}

#[allow(dead_code)]
impl Intergral {
    fn calc(&self) {
        // this assumes the step to be 1, this is a problem for resolution of the integral
        // this is using unsigned integers and nows the math is all fucked up I need to refactor
        // this
        // fuck
        // asdasdsdads
        // fuck

        print!("outside iter\n");

        let mut area: f32 = 0.0;

        std::iter::successors(Some(self.lower_limit), move |&prev| {
            let next = prev + 0.011011;

            print!("In iter\n");

            let part_a1: f32 =
                (self.parsed_func.a.constant as f32) * (next.powf(self.parsed_func.a.power as f32));
            let part_b1: f32 =
                (self.parsed_func.b.constant as f32) * (next.powf(self.parsed_func.b.power as f32));
            let part_c1: f32 = self.parsed_func.c.constant as f32;

            let part_a2: f32 = (self.parsed_func.a.constant as f32)
                * ((next - prev).powf(self.parsed_func.a.power as f32));
            let part_b2: f32 = (self.parsed_func.b.constant as f32)
                * ((next - prev).powf(self.parsed_func.b.power as f32));
            let part_c2: f32 = self.parsed_func.c.constant as f32;

            //print!("part_c {}\n", &part_c);
            //print!("part_c const: {}, pow: {}\n", self.parsed_func.c.constant, self.parsed_func.c.power);
            //print!("i {}\n", i);
            //print!("part_a {}\n", part_a);

            /*
            print!(
                "[*] calculating function points!!! x = {} , y = {}\n",
                &i, &last_part
            );
            */

            let d1: f32 = next - (next - prev);
            let y1: f32 = part_a1 + part_b1 + part_c1;
            let y2: f32 = part_a2 + part_b2 + part_c2;

            let a: f32 = (2.0 * (d1)) * (2.0 * (y1));
            let b: f32 = -(d1 * y1);
            let c: f32 = d1 * y2;

            area = area + (a + b + c) / 2.0;

            print!(
                "X: {},\nY1: {},\nY2: {},\nArea: {}\n",
                &next, &y1, &y2, &area
            );

            (next < self.higher_limit).then_some(next)
        });

        return;
    }

    fn set_bounds(&mut self, low: f32, high: f32) {
        self.higher_limit = high;
        self.lower_limit = low;
    }
}

#[allow(dead_code)]
//assume the form cv^p (constant, variable, power)
struct PartialFunc {
    constant: u32,
    variable: u32,
    power: u32,
}

#[allow(dead_code)]
//assume the form ax^2+bx+c
struct ParsedFunc {
    a: PartialFunc,
    b: PartialFunc,
    c: PartialFunc,
}

#[allow(dead_code)]
//assume the form int(func(x))d(x)
struct Intergral {
    lower_limit: f32,
    higher_limit: f32,
    parsed_func: ParsedFunc,
}

fn parse_to_partial(arg: &str) -> PartialFunc {
    print!("[*] args passed to partial {}\n", arg);

    // this is fucked up, converting ascii -> real value.
    // this is funny lol
    let mut con: u32 = (arg.chars().nth(0).unwrap() as u32) - 48;
    let mut var = 1;
    let mut pow = 1;

    if arg.len() == 4 {
        con = (arg.chars().nth(0).unwrap() as u32) - 48;
        var = 0;
        pow = (arg.chars().nth(3).unwrap() as u32) - 48;
    } else if arg.len() == 2 {
        con = (arg.chars().nth(0).unwrap() as u32) - 48;
        var = 0;
        pow = 1;
    }

    // this is so fucky
    let p = PartialFunc {
        constant: con,
        variable: var,
        power: pow,
    };

    print!("[*] p.constant is equal to: {}\n", &p.constant);
    print!("[*] p.variable is equal to: {}\n", &p.variable);
    print!("[*] p.power is equal to: {}\n", &p.power);

    return p;
}

fn parse_args_to_func(args: &Vec<String>) -> ParsedFunc {
    let func = &args[1]; // assuming the function is here

    let parts: Vec<_> = func.split('+').collect();

    dbg!("{}\n", &parts);

    let func = ParsedFunc {
        a: parse_to_partial(parts.iter().nth(0).unwrap()),
        b: parse_to_partial(parts.iter().nth(1).unwrap()),
        c: parse_to_partial(parts.iter().nth(2).unwrap()),
    };

    return func;
}
