/*By using the learned concepts, write a function with a proper signature that ...
 ... calculates the absolute value of a number.
 ... enforces a number to be in a given range.
 ... determines the maximum of three numbers.
 ... determines the amount of days for a given month.
 ... prints the grade for this course based on four test results.
 ... prints the body mass index category for a given weight and height.
 ... calculates the factorial of a given number (advanced) */

/* By using the learned concepts, write a function with a proper signature that ...
 ... mutates a number by adding another number n.
 ... concatenates two strings.
 ... swaps the values of two variables.
 ... returns a nicely formatted string congratulation for a given birthday.
 ... repeats a string n times, thus producing a new string (advanced).
 ... reverses the digits of a positive integer (advanced) */

 /* By using the learned concepts, write a function with a proper signature that ...
 ... finds the minimum and the sum of a collection of numbers.
 ... prints the body mass index category (2nd exercise) using pattern matching.
 ... returns the longest prefix of a collection having numbers smaller than n.
 ... returns all prime numbers up to a given number n.
 ... sorts a collection of people with names and ages by name.
 ... prints a collection of heroes with names and health points in a nice way by
 visualizing them below each other with their health points as health bars.
 ... plans the ultimate heist in preparation for GTA VI, picking the best
 combination of items to put into a bag of capacity k, where each item has
 some value v and some weight w (advanced) */

 /*By using the learned concepts, write types/functions/methods that ...
 ... calculate the area and circumference of a graphical object, which can
 either be a rectangle, a right triangle with sides of equal length, or a circle.
 ... visualize such a graphical object using print statements.
 ... stepwise build and manipulate a Sudoku board.
 ... visualize such a Sudoku board using print statements.
 ... improve the solutions from last time in terms of readability, reliability, etc.
 ... solve a Sudoku board (advanced). */

/*By using the learned concepts, write a function with a proper signature that ...
• ... finds the minimum of two values in a tuple.
• ... sums up a collection of optional numeric values.
• ... finds the maximum of a collection of elements having any type.
• ... finds an element in a collection of any type and returns its index.
• ... sorts a collection of elements having any type (swap method is allowed).
• ... joins elements of any type into a string using a given delimiter.
• ... evaluates an arithmetic expression (numbers, addition, subtraction) given
as string and returns different kinds of errors on incorrect inputs (advanced). */

/* 1-Write a function that finds the maximum and the sum of a collection of numbers.
2-Write a function that returns all prefixes of a collection (pick any type for the elements within the collection).
3-Write a function that rotates a collection of string literals n times to the left. 
Rotating a collection one time to the left means that every element of the collection is moved one place (index) to the left, 
where the leftmost element becomes the last element.
4-Write a function that converts a String of digits into a number. Tolerate whitespaces at the beginning and the end of the String. */


fn abs_val (x:i32)->i32{
    if x < 0 {
        x*-1
    } else {
        x
    }
    
}

fn enforce(number:i32)->i32{
    if number > 100 {
        100
    } else if number < 0{
        0
    } else {
        number
    }

}


fn max_of_three(num1:i32,num2:i32,num3:i32)->i32{
    if num1> num2{
        num1
    } else if num1<num3{
        num3
    }else {
        num2
    }
}

fn days_in_month(month:u32)->u32{
     if
        month == 1 ||
        month == 3 ||
        month == 5 ||
        month == 7 ||
        month == 8 ||
        month == 10 ||
        month == 12
    { // when we know a month has 31 days we return 31
        31
    } else if month == 4 || month == 6 || month == 9 || month == 11 { // when we know a month has 30 days we return 30
        30
    } else {
        28
}
}

fn grading(test1:f32,test2:f32,test3:f32,test4:f32)-> u32 {

    let grading = test1*0.10+test2*0.20+test3*0.35+test4*0.35;

    if grading >= 90.0 {
        1
    } else if grading >= 80.0 {
        2
    }else if grading >= 70.0 {
        3
    }else if grading >= 60.0 {
        4
    }else {
        5
    }
}

fn bmi(weight:f32, height:f32)->f32{
    weight / (height * height)
}
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;

    while i <= n {
        result *= i;
        i += 1;
    }

    result
}

fn mut_nuber(){

}
fn concat_str(){

}

fn swap_val(){

}
fn birthday(){
    
}

fn repeat_string(){

}

fn reversing_digits(){

}

fn find_min(list: &[i32])-> (i32, &str){
    if list.len() == 0 {
        return (0, "The length of the input is zero.");
    }else {
       let  mut min = list[0];
       /* 
        for n in list {
            if *n < min {
                min = *n;
            } 
        } 

        */

        for i in 1..list.len(){
            if list[i] < min {
                min = list[i];
            }
        }
        return (min , "is the minimum");
    }
    //return (9, 10,11);

}
fn find_sum(list: &[i32])->i32{
todo!();
}


fn main(){
    //calculates the absolute value of a number.
    println!("{}",abs_val(-10));
    
    //enforces a number to be in a given range.
    println!("{}", enforce(101));

    //determines the maximum of three numbers.
    println!("{}",max_of_three(5, 10, 15));

    //determines the amount of days for a given month.
    println!("{}", days_in_month(2));

    //prints the grade for this course based on four test results.
    println!("{}", grading(100.0, 100.0, 100.0, 100.0));

    //prints the body mass index category for a given weight and height.
    println!("{}",bmi(65.7, 1.70));

    //calculates the factorial of a given number (advanced) 
    println!("{}", factorial(0)); 

    //mutates a number by adding another number n.

    
    //concatenates two strings.

    
    //swaps the values of two variables.


    //returns a nicely formatted string congratulation for a given birthday.


    //repeats a string n times, thus producing a new string (advanced).


    //reverses the digits of a positive integer (advanced) 

    //finds the minimum and the sum of a collection of numbers.
    let array = &[12,31,6]; 
    println!("{:?}", find_min(array));
    

}