fn main(){
    let test = FloatingPointRange::calc_steps(-0.3, 1.0, 0.1);

    print!("Test: {}\n", test); 

    return;
}

#[allow(dead_code)] 
struct FloatingPointRange{
    start: f64, 
    end: f64, 
    step: f64,
} 

#[allow(dead_code)] 
impl FloatingPointRange{
    pub fn calc_steps(start: f64, end: f64, step: f64) -> u64 {
        let steps: u64 = ((end - start) / step).abs().round() as u64;
        return steps; 
    }
}
