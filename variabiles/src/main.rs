// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }


// use std::io;
// use std::process;

// fn main() {
//     let a = [1, 2, 3, 4, 5];
    
//     println!("Please enter an array index.");

//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read linerror");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
    
//     match index {
//         0..=4 => println!("inside"),
//         _ => {
//             println!("outside");
//             process::exit(404);
//         }
//     }

//     let element = a[index];

//     println!(
//         "The value of the element at index {} is: {}",
//         index, element
//     )
// }

fn main() {
    println!("hello world!");

    another_function();
}

// Function definitions in Rust start with fn
fn another_function() {
    println!("Another function.")
}