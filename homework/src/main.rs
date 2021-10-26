use std::io;
use std::process;

fn main() {
    // convert_f2c();
    temp_convert();
}

fn temp_convert() {
    let mut opt = String::new();
    println!("
        which one do you want:
            1. convert Fahrenheit to Celsius
            2. convert Celsius to Fahrenheit
    ");
    io::stdin()
        .read_line(&mut opt)
        .expect("Failed to read input_number");
    
    let opt: u32 = opt.trim().parse().expect("Failed to parse");

    if (1..2).contains(&opt) {
        println!("please type temperature number: ");
        let mut temperature = String::new();
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read tem_number");
        let temperature: i32 = temperature.trim().parse().expect("Failed to parse to temp_number");

        let convert_temperature = 
        if opt == 1 { 
            convert_f2c(temperature) 
        } else { 
            convert_c2f(temperature) 
        };

        println!("the convert temperature is: {}", convert_temperature);
        
    } else {
        println!("invalid type!");
        process::exit(0);
    }
}

// C to F: F = C*(9/5) + 32
// F to C: C = (F-32)*(5/9)

// Convert temperatures between Fahrenheit and Celsius
// Fahrenheit: /ˈfar(ə)nhʌɪt/ 华氏度
// Celsius /ˈsel.si.əs/ 摄氏度
// 摄氏度 = （华氏度 - 32）/ 1.8 或者（华氏度 - 32）/ (5/9)
// 摄氏度是从0c-100c，华氏度是32f - 212f,而华氏度每180度温差变化相当于摄氏度100度温差变化（1.8f = 1c)，所以还要除以1.8（1.8相当于五分之九）
fn convert_f2c(fahrenheit: i32) -> i32 {
    let celsius: i32 = fahrenheit - 32;
    let celsius: f32 = celsius as f32 / 1.8;

    celsius as i32
}

fn convert_c2f(celsius: i32) -> i32 {
    let fahrenheit: i32 = celsius * (9 / 5) + 32;
    fahrenheit
}
