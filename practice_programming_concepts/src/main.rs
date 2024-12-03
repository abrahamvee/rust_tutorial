fn main() {
    // Temperature conversion
    let t_celsius = 5;
    let t_farenheit = 70;
    println!("Celsius: {} to Farenheit is {}", t_celsius, c_to_f_conversion(t_celsius));
    println!("Farenheit: {} to Celsius is {}", t_farenheit, f_to_c_conversion(t_farenheit));

    //Print fibonacci sequence
    let n_fib_num = 10;
    let mut a = 0;
    let mut b = 1;

    for _ in 0..n_fib_num{
        println!("{}", a);

        let temp = a;
        a = b;
        b = temp + b;
    }


    //Twelve days of christmas
    let gifts = [
        "a Partridge in a Pear Tree",
        "two Turtle Doves",
        "three French Hens",
        "four Calling Birds",
        "five Golden Rings",
        "six Geese a Laying",
        "seven Swans a Swimming",
        "eight Maids a Milking",
        "nine Ladies Dancing",
        "ten Lords a Leaping",
        "eleven Pipers Piping",
        "twelve Drummers Drumming",
    ];

    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    for day in 0..12{
        println!("On the {} day of Christmas, my true love gave to me:",
        days[day]);

        for gift in (0..=day).rev(){
            if day > 0 && gift == 0 {
                print!("and ");
            }
            println!("{}", gifts[gift]);
        }
        println!();
    }
} 
 

fn c_to_f_conversion(c: i32)->f32{
        // No need for return since it is an expression and it will return
        (c as f32) * (9.0/5.0) + 32.0
}

fn f_to_c_conversion(f: i32)->f32{
    ((f as f32) - 32.0) * (5.0/9.0)
}

fn gen_fibonacci_sequence(n: i32){

}
