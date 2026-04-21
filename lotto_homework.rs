/* 
Lotto is a popular gambling game in Austria. The game is pretty simple:

First, the player picks n unique numbers from a range of numbers, for example 6 numbers between 1 and 45.
The gambling company then randomly picks n+1 unique numbers from the same range (n normal numbers plus one bonus number).
Based on the numbers that were guessed correctly, the player may win a prize. For the 6-out-of-45 system, the prize ranks are usually like this:
Prize rank	Correctly guessed numbers
1	6 normal numbers
2	5 normal numbers + bonus number
3	5 normal numbers
4	4 normal numbers + bonus number
5	4 normal numbers
6	3 normal numbers + bonus number
7	3 normal numbers
8	0 normal numbers + bonus number
Your task is to write a program that allows a user to play Lotto from a player's perspective. That means:

When the program starts, the user has to pick n unique numbers from a pre-defined range of numbers.
The system then randomly generates n+1 unique numbers from the same range.
The program prints the user-picked numbers in ascending order.
The program prints the randomly generated numbers in ascending order, as well as the bonus number.
The program determines the achieved prize rank and prints it, together with a message about the correctly guessed numbers.
Pay attention to the following conditions:

The order of the numbers (both guessed and randomly generated) is irrelevant for determining the prize rank.
As always, use only the programming concepts you have learned so far.
Make sure that the user can re-enter erroneous inputs (e.g., when entering an invalid number, an out-of-range number or a duplicate number). Provide a meaningful error message for the user when encountering an erroneous input.
Make sure to tolerate imprecise user inputs, if reasonable (e.g., whitespaces before and/or after number inputs).
Write your program in a way such that the count of guessed numbers n and the range of numbers can easily be changed.
You are allowed to use the code below for reading user input and generating random numbers. It should work for both the offline Rust compiler and the Rust Playground. When using the Rust Playground, you can provide input to the program by using the textbox and "Send" button at the bottom right when running your program.
*/
use rand::Rng;
use std::io::stdin;

fn main() {
    // Read input from user.
    let mut input = String::new();
    let stdin = stdin();
    stdin.read_line(&mut input).unwrap();
    println!("Your input was: {input}");

    // Generate a random integer (0 <= i < 100).
    let mut generator = rand::rng();
    let number = generator.random_range(0..100);
    println!("Your generated number: {number}");
} 


