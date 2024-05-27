/*
This program is a test of my own skill in making a simple program in Rust to convert user input.
The program first allows the user to select what type of temperature conversion they wish to perform.
After selecting the type of temperature conversion, the user then enters the temperature they wish to convert.
The program displays the results of the temperature conversion.
The program is designed to loop until the user indicates they no longer wish to use the program.
*/
use std::io;

fn main() {
    println!("Welcome to the temperature conversion program.");
    loop {
        println!("Please enter 1 to convert from Celsius to Fahrenheit or 2 for the opposite or 3 to exit:");

        let mut program_choice = String::new();

        io::stdin() //using standard input so the user can enter anything
            .read_line(&mut program_choice)
            .expect("Failed to read the line");

        //The code below ensures the input from the user is a number and
        //does not crash the program if the input is not a number
        let program_choice: u32 = match program_choice.trim().parse() {
                Ok(num) => num, //move on to the next step of the program
                Err(_) => continue, //repeat the loop since the input was not within
                //the range of appropriate choices
            };

        if program_choice == 1 { //Using two separate functions to do the calculations
            celsius_to_fahrenheit();
        }
        else if program_choice == 2 {
            fahrenheit_to_celsius();
        }
        else if program_choice == 3{
            break;
        }
    }
}

//function to convert celsius to fahrenheit
fn celsius_to_fahrenheit(){
    println!("Please enter the temperature you would like to convert to Fahrenheit:");
    let mut celsius_temp = String::new(); //input is a string
    io::stdin().read_line(&mut celsius_temp).expect("Failed to read the line");
    //The part below is to ensure the temperature entered is a number
    let celsius_temp: u32 = celsius_temp //convert the string to a number and remove any extra whitespace
        .trim() //ignore whitespace
        .parse() //convert to integer
        .expect("Please restart the program!");
    let celsius_fahrenheit = (celsius_temp * 9/5) + 32;
    println!("The temperature in Fahrenheit is: {}", celsius_fahrenheit);

}

//function to convert fahrenheit to celsius
fn fahrenheit_to_celsius(){
    println!("Please enter the temperature you would like to convert to Celsius:");
    let mut fahrenheit_temp = String::new(); //input is a string
    io::stdin().read_line(&mut fahrenheit_temp).expect("Failed to read the line");
    //The part below is to ensure the temperature entered is a number
    let fahrenheit_temp: u32 = fahrenheit_temp //convert the string to a number and remove any extra whitespace
        .trim() //ignore whitespace
        .parse() //convert to integer
        .expect("Please restart the program!!");
    let fahrenheit_celsius = (fahrenheit_temp - 32) * 5/9;
    println!("The temperature in Celsius is: {}", fahrenheit_celsius);
}