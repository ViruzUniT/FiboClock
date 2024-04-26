mod test;
extern crate colored;

use crate::colored::*;
use std::io::{stdin, stdout, Write};

// A Color Enum for the Fields
// With the colors
// White/None
// Red/hour
// Green/minute
// Blue/Hour and minute
#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    None,
    Red,
    Green,
    Blue,
}

// Represents a field in the Fibonacci Clock
#[derive(Debug, Clone, Copy)]
struct Field {
    value: i32,
    color: Color,
}

impl Field {
    // Just a basic Field Constructor
    fn new(value: i32) -> Field {
        return Field {
            value,
            color: Color::None,
        };
    }
}

// Just a Time struct to store time
// Takes a i16 for the hour and for the minute
#[derive(Debug)]
struct Time {
    hour: i32,
    minute: i32,
}

impl Time {
    fn new() -> Time {
        return Time { hour: 0, minute: 0 };
    }
}

// Gets the time of an 5 long Field array
// This function returns a Result
// If the passed array is empty too short or too long it will return an error
fn get_time(fields: &[Field; 5]) -> Result<Time, &str> {
    let mut hour: i32 = 0;
    let mut minute: i32 = 0;

    if fields.len() == 0 {
        return Err("fields empty");
    } else if fields.len() != 5 {
        return Err("There are not enough or too many fields");
    }

    for i in 0..fields.len() {
        let mut adder: i32 = 0;
        if i == 0 || i == 4 {
            adder = 1
        }

        match fields[i].color {
            Color::None => (),
            Color::Red => {
                hour += (i) as i32 + adder;
            }
            Color::Green => {
                minute += ((i) as i32 + adder) * 5;
            }
            Color::Blue => {
                hour += (i) as i32 + adder;
                minute += ((i) as i32 + adder) * 5;
            }
        }
    }
    Ok(Time { hour, minute })
}

// prints the Welcome message with the rules
fn print_start() {
    println!("{}", "Welcome to the Fibonacci Clock setter".green().bold());
    println!("{}:", "Here is how it works".cyan().bold());
    println!(
        "1. {}",
        "You pick the slot you want to edit".cyan().italic()
    );
    println!(
        "2. {} None {} {} {}",
        "You pick the Color, the Colors are".cyan().italic(),
        "Red".red(),
        "Green".green(),
        "Blue".blue()
    );
    println!("3. {}", "When you picked the colors for the fields, I will tell you the time and maybe show you your customized clock".cyan().italic());
}

// removes \n or \r from the string
fn string_cleaner(string: &mut String) {
    if let Some('\n') = string.chars().next_back() {
        string.pop();
    }
    if let Some('\r') = string.chars().next_back() {
        string.pop();
    }
    if let Some(' ') = string.chars().next_back() {
        string.pop();
    }
}

// just an shortcut to get an input
fn get_input() -> String {
    let mut input = String::new();
    let _ = stdout().flush();
    stdin()
        .read_line(&mut input)
        .expect("Your input does not work somehow");
    string_cleaner(&mut input);
    return input;
}

// let the user say he Understood the rules
fn get_confirm() {
    loop {
        println!(
            "{} (Type y/n)",
            "Understood?".red().bold().underline().blink()
        );
        let input: String = get_input();

        if input == "y" {
            break;
        }
    }
}

// transfers the color values of the fields into UTF-8 Blocks
fn transfer_color(fields: [Field; 5]) -> [char; 5] {
    let red_sqare = '🟥';
    let green_sqare = '🟩';
    let blue_sqare = '🟦';
    let white_sqare = '⬜';

    let mut fields_color: [char; 5] = [white_sqare; 5];

    for i in 0..5 {
        match fields[i].color {
            Color::Red => {
                fields_color[i] = red_sqare;
            }
            Color::Green => {
                fields_color[i] = green_sqare;
            }
            Color::Blue => {
                fields_color[i] = blue_sqare;
            }
            Color::None => {
                fields_color[i] = white_sqare;
            }
        }
    }
    return fields_color;
}

// just print i should remove it but its to much work
fn print_field(field: char) {
    print!("{}", field);
}

// a VERY ugly way to print the clock 'UI'
fn print_clock(fields: [Field; 5]) {
    let fieldlength = [2, 2, 4, 6, 10];

    let colored_fields = transfer_color(fields);

    for _ in 0..fieldlength[0] {
        for _ in 0..fieldlength[2] {
            print_field(colored_fields[2]);
        }
        for _ in 0..fieldlength[0] {
            print_field(colored_fields[0]);
        }
        for _ in 0..fieldlength[4] {
            print_field(colored_fields[4]);
        }
        print!("\n");
    }
    for _ in 0..fieldlength[1] {
        for _ in 0..fieldlength[2] {
            print_field(colored_fields[2]);
        }
        for _ in 0..fieldlength[1] {
            print_field(colored_fields[1]);
        }
        for _ in 0..fieldlength[4] {
            print_field(colored_fields[4]);
        }
        print!("\n");
    }
    for _ in 0..fieldlength[3] {
        for _ in 0..fieldlength[3] {
            print_field(colored_fields[3]);
        }
        for _ in 0..fieldlength[4] {
            print_field(colored_fields[4]);
        }
        print!("\n");
    }
}

// prints the with colors entered time
fn print_time(fields: [Field; 5]) {
    let time = get_time(&fields).unwrap();
    println!(
        "hour: {} minutes: {}",
        &time.hour.to_string().red().bold(),
        &time.minute.to_string().red().bold()
    );
    print_clock(fields);
}

// modifies a given field
fn modify_field(field: &mut Field, string: &str) {
    match string {
        "red" => {
            field.color = Color::Red;
        }
        "green" => {
            field.color = Color::Green;
        }
        "blue" => {
            field.color = Color::Blue;
        }
        "none" => {
            field.color = Color::None;
        }
        _ => panic!("you did some big shit"),
    }
}

// prints the info text for the pick/confirm screen
fn print_pick_info() {
    println!(
        "{} {} {}",
        "Ok now pick a Field from 1-5 or just confirm to get the time or type"
            .cyan()
            .bold(),
        "exit".red().bold(),
        "to exit".cyan().bold()
    );
    println!("{}", "1/2 is 1\n3 is 2\n4 is 3\n5 is 5".green().bold());
    println!(
        "{} none, {}, {}, {}",
        "Colors:".cyan().bold(),
        "red".red().bold(),
        "green".green().bold(),
        "blue".blue().bold()
    );
}

// user can pick a field, confirm or exit
fn pick_or_confirm() {
    let mut fields: [Field; 5] = [
        Field::new(1),
        Field::new(1),
        Field::new(2),
        Field::new(3),
        Field::new(5),
    ];
    let mut picked_field: &mut Field;
    loop {
        print_pick_info();
        let mut input: String = get_input();

        match &input[..] {
            "" => {
                print_time(fields);
                continue;
            }
            "exit" => {
                break;
            }
            _ => (),
        }
        let parsed_input = input.parse::<i16>();
        match parsed_input {
            Ok(selecetion) => {
                if selecetion < 1 || selecetion > 5 {
                    println!("{}", "You picked something wrong".red().bold().underline());
                    continue;
                }
                picked_field = &mut fields[selecetion as usize - 1];
            }
            Err(err) => {
                println!(
                    "{} ({})",
                    "You picked something wrong".red().bold().underline(),
                    err.to_string().bold().red()
                );
                continue;
            }
        }

        println!("{}", "Ok now pick the color".cyan().bold());

        input = get_input();
        match &input[..] {
            "red" => (),
            "green" => (),
            "blue" => (),
            "none" => (),
            _ => {
                println!(
                    "{}",
                    "You picked a non existing color".red().bold().underline()
                );
                continue;
            }
        }
        modify_field(picked_field, &input);
        println!("{}\n", "Color picked".cyan().bold());
    }
}

// asks the user if you want to use the time to clock or color to time part of the program
fn print_real_start() -> bool {
    println!("{}", "Do you want to\n\t1. Create a clock\n\t2. Give a clock some colors and get the time\nPls awnser with \"1\" or \"2\"".cyan().bold());
    loop {
        let input = get_input();
        match &input[..] {
            "1" => {
                return true;
            }
            "2" => {
                return false;
            }
            _ => {
                println!("{}", "Wrong pick".red().bold().underline());
            }
        }
    }
}

// asks the user for a time thats between 0 and the "max_time"
fn get_user_time(max_time: i32) -> Option<i32> {
    let input = get_input();

    let parsed_input = input.parse::<i32>();
    match parsed_input {
        Ok(number) => {
            if number >= 0 && number <= max_time {
                return Some(number);
            }
            println!("{}", "wrong number".red().bold(),);
            return None;
        }
        Err(err) => {
            println!(
                "{} ({})",
                "no number".red().bold(),
                err.to_string().red().bold()
            );
            return None;
        }
    }
}

// transfers the given time to the color in the fields
fn time_to_color(field_list: &mut [Field; 5], time: i32, is_hours: bool) {
    let mut remainder = if is_hours { time } else { time / 5 };
    let mut color: Color = Color::Red;

    for i in (0..field_list.len()).rev() {
        if !is_hours {
            color = Color::Green;
        }
        if field_list[i].color != Color::None && field_list[i].color != color {
            color = Color::Blue;
        }

        if field_list[i].color == color {
            continue;
        }
        if remainder >= field_list[i].value && field_list[i].value != 4 {
            remainder -= field_list[i].value;
            field_list[i].color = color;
        } else if remainder >= 4 && field_list[i].value == 4 {
            remainder -= field_list[i].value;
            field_list[1].color = color;
            field_list[i - 1].color = color;
        }
    }
}

// gets the hour as input from the user
fn get_hour_input(time: &mut Time) {
    loop {
        println!("{}", "Pls input a hour from 0 - 12".cyan().bold());
        let time_input = get_user_time(12);
        if time_input != None {
            time.hour = time_input.unwrap();
            break;
        }
    }
}

// gets the minute as input from the user
fn get_minute_input(time: &mut Time) {
    loop {
        println!(
            "{}",
            "Pls input a minute from 0 - 55 and pls only values that are divisible by 5"
                .cyan()
                .bold()
        );
        let time_input = get_user_time(55);
        if time_input != None && time_input.unwrap() % 5 == 0 {
            time.minute = time_input.unwrap();
            break;
        }
    }
}

// gets the input for the time and prints the clock
fn print_inputed_time() {
    loop {
        let mut time = Time::new();

        get_hour_input(&mut time);
        get_minute_input(&mut time);

        let mut fields = [
            Field::new(1),
            Field::new(1),
            Field::new(2),
            Field::new(3),
            Field::new(5),
        ];
        time_to_color(&mut fields, time.hour, true);
        time_to_color(&mut fields, time.minute, false);
        print_clock(fields);
    }
}

// main
fn main() {
    if !print_real_start() {
        print_start();
        get_confirm();
        pick_or_confirm();
    } else {
        print_inputed_time();
    }
}
