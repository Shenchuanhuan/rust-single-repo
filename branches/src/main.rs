fn main() {
    let number = 3;

    // if expression
    // You must be explicit and always provide if with a Boolean as its condition
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let other_number = 6;
     
    if other_number % 6 == 0 {
        println!("number is divisible by 6");
    } else if other_number % 3 == 0 {
        println!("number is divisible by 3");
    } else if other_number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 6, 3 or 2");
    }

    // using if in let statement
    let condition = true;
    let con_number = if condition { 5 } else { 6 };

    println!("The value of con_number is: {}", con_number);

    loop_main();

    return_loop();

    while_loop();

    while_example_loop();

    for_loop();

    other_for_loop();
}

// Rust has three kinds of loops: loop, while, and for.

// loop: You can place the break keyword within the loop to tell the program when to stop executing the loop.
fn loop_main() {
    // loop {
    //     println!("again!");
    // }
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

// while loop
fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn while_example_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

// for loop
fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("this value is: {}", element);
    }
}

fn other_for_loop() {
    // rev means reverse
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}