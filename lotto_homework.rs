/*Lotto is a popular gambling game in Austria. The game is pretty simple:

First, the player picks n unique numbers from a range of numbers, for example 6 numbers
between 1 and 45.
The gambling company then randomly picks n+1 unique numbers from the same range (n normal numbers
plus one bonus number).
Based on the numbers that were guessed correctly, the player may win a prize. For the 6-out-of-45 
system,the prize ranks are usually like this:
Prize rank	Correctly guessed numbers
1	6 normal numbers
2	5 normal numbers + bonus number
3	5 normal numbers
4	4 normal numbers + bonus number
5	4 normal numbers
6	3 normal numbers + bonus number
7	3 normal numbers
8	0 normal numbers + bonus number
Your task is to write a program that allows a user to play Lotto from a player's perspective.
That means:

When the program starts, the user has to pick n unique numbers from a pre-defined range of numbers.
The system then randomly generates n+1 unique numbers from the same range.
The program prints the user-picked numbers in ascending order.
The program prints the randomly generated numbers in ascending order, as well as the bonus number.
The program determines the achieved prize rank and prints it, together with a message about the correctly
guessed numbers.Pay attention to the following conditions:

The order of the numbers (both guessed and randomly generated) is irrelevant for determining the prize rank.
As always, use only the programming concepts you have learned so far.
Make sure that the user can re-enter erroneous inputs (e.g., when entering an invalid number, 
an out-of-range number or a duplicate number). Provide a meaningful error message for the user 
when encountering an erroneous input.
Make sure to tolerate imprecise user inputs, if reasonable (e.g., whitespaces before and/or 
after number inputs).
Write your program in a way such that the count of guessed numbers n and the range of numbers
can easily be changed.
You are allowed to use the code below for reading user input and generating random numbers. 
It should work for both the offline Rust compiler and the Rust Playground. When using the Rust 
Playground, you can provide input to the program by using the textbox and "Send" button at the 
bottom right when running your program.
*/
use rand::RngExt;
use std::io::stdin;

fn contains_number(numbers: &Vec<u32>, number: u32) -> bool {
 
    for i in 0..numbers.len(){
    
        if numbers[i] == number{
        
            return true;
            
        }
        
    }
    
    false
}

fn to_number(input: String) -> u32 {

    let mut number = 0;

    for c in input.chars() {
        
        if c >= '0' && c <= '9'{
        
            let result = c as u32 - '0' as u32;
        
            number = number * 10 + result;
        }
        
    }

    number
}

fn read_number() -> u32 {

    let mut input = String::new();
   
    let stdin = stdin();
    
    stdin.read_line(&mut input).unwrap();
    
    let number = to_number(input);
    
    number
}

fn read_numbers(n: usize, max: u32) -> Vec<u32> {

    let mut numbers = Vec::new();
    
    while numbers.len() < n {
    
        let number = read_number();
    
        if !(contains_number(&numbers, number)) && number > 0 && number <= max {
            numbers.push(number)
        }
    }
    
    numbers
    
}

fn generate_numbers(n: usize, max: u32) -> Vec<u32> {
    
    let mut generator = rand::rng();
    
    let mut numbers = Vec::new();
    
    
    
    while numbers.len() < n {

    let number = generator.random_range(1..=max);

    if !(contains_number(&numbers, number)) {
            numbers.push(number)
        }
    }
    numbers
}

fn sort_numbers(numbers:Vec<u32>)->Vec<u32>{
    
    let mut result = Vec::new();
    
    for n in numbers{
        result.push(n)
    }
    
    for _ in 0..result.len(){
        for i in 0..result.len()-1{
            if result[i] > result[i+1]{
                let temp = result[i];
                result[i] = result[i+1];
                result[i+1] = temp;
            }
        }
    }
    result
}
fn count_correct(player: &Vec<u32>, computer: &Vec<u32>) -> u32 {
    
    let mut count = 0;
    
    for p in player {
        for c in computer{
            if p == c {
                count +=1;
            }
        }
    }
    count
}

fn prize_rank(correct: u32, bonus_correct: bool) -> u32 {
    
    if correct == 6 {
        1
    } else if correct == 5 && bonus_correct{
        2
    } else if correct == 5 {
        3
    } else if correct == 4 && bonus_correct{
        4
    } else if correct == 4 {
        5
    } else if correct == 3 && bonus_correct{
        6
    } else if correct == 3 {
        7
    } else if correct == 0 && bonus_correct {
        8
    } else {
        0
    }
}

fn main() {

    let player_num = read_numbers(6, 45);
    
    let comp_num = generate_numbers(7, 45);
    
    let bonus = comp_num[comp_num.len() - 1];
    
    let mut normal_numbers = Vec::new();

    for i in 0..6 {
        normal_numbers.push(comp_num[i])
    }
    
    let correct = count_correct(&player_num, &normal_numbers);
    
    let bonus_correct = contains_number(&player_num, bonus);
    
    let rank = prize_rank(correct, bonus_correct);
    
    println!("Player numbers: {:?}", sort_numbers(player_num));
    
    println!("Computer numbers: {:?}", sort_numbers(normal_numbers));
    
    println!("Bonus number: {:?}", bonus);
    
    println!("Correct normal numbers: {}", correct);
    
    println!("Bonus guessed correctly: {}", bonus_correct);
    
    println!("Prize rank: {}", rank);
}
