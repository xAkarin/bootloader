fn main(){
    let test = FloatingPointRange::calc_steps_rounded(-0.3, 1.0, 0.1);

    print!("Test: {}\n", test); 

    return;
}

#[allow(dead_code)] 
struct FloatingPointRange{
    start: f64, 
    end: f64, 
    step: f64,
    current: f64,
    next: f64,
} 

#[allow(dead_code)] 
impl FloatingPointRange{
    pub fn calc_steps_rounded(start: f64, end: f64, step: f64) -> u64 {
        return
            ((end - start) / step).abs().round() as u64; 
    }
    
    pub fn calc_steps(start: f64, end: f64, step: f64) -> f64 {
        return
            ((end - start) / step).abs();  
    }
}

impl Iterator for FloatingPointRange{
    type Item = f64; 

    fn next(&mut self) -> Option<Self::Item> { todo!() }

}
