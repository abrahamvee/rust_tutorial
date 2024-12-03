fn main() {
    // Temperature conversion
    let t_celsius = 5;
    let t_farenheit = 70;
    println!("Celsius: {} to Farenheit is {}", t_celsius, c_to_f_conversion(t_celsius));
    println!("Farenheit: {} to Celsius is {}", t_farenheit, f_to_c_conversion(t_farenheit));
} 
 
fn c_to_f_conversion(c: i32)->i32{
        // No need for return since it is an expression and it will return
        (c * (9/5))/32
}

fn f_to_c_conversion(f: i32)->i32{
    (f - 32) * (5/9)
}

