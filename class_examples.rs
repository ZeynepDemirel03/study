

use std::ops::AddAssign;



//calculates the absolute value of a number.
fn abs_val (x:i32)->i32{
    if x < 0 {
        x*-1
    } else {
        x
    }
    
}


//enforces a number to be in a given range.
fn enforce(number:i32)->i32{
    if number > 100 {
        100
    } else if number < 0{
        0
    } else {
        number
    }

}


//determines the maximum of three numbers.
fn max_of_three(num1:i32,num2:i32,num3:i32)->i32{
    if num1> num2{
        num1
    } else if num1<num3{
        num3
    }else {
        num2
    }
}


//determines the amount of days for a given month.
fn days_in_month(month:u32)->u32{
     if
        month == 1 ||
        month == 3 ||
        month == 5 ||
        month == 7 ||
        month == 8 ||
        month == 10 ||
        month == 12
    { 
        31
    } else if month == 4 || month == 6 || month == 9 || month == 11 { 
        30
    } else {
        28 //i was too lazy to do rest :p
}
}



//prints the grade for this course based on four test results.
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



//prints the body mass index category for a given weight and height.
fn bmi(weight:f32, height:f32)->f32{
    weight / (height * height)
}



//calculates the factorial of a given number (advanced) tekrar yap
fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;

    while i <= n {
        result *= i;
        i += 1;
    }

    result
}


//mutates a number by adding another number n.
fn mut_nuber(org_num: &mut i32, n:i32)->i32{
    *org_num += n;
    n

}

//concatenates two strings.
fn concat_str(text1: String, text2:String)-> String{
    text1 + &text2
}


//swaps the values of two variables.
fn swap_val(a:&mut i32 , b:&mut i32){
    let change = *a;
    *a = *b;
    *b = change;
}


//returns a nicely formatted string congratulation for a given birthday.
fn birthday(age:u8)->String {
    format!(
        "{:*^30}\n{:^^30}\n{:*^30}",
        "HAPPY",
        format!("{age}th"),
        "BIRTHDAY"
    )
}


//repeats a string n times, thus producing a new string (advanced).
fn repeat_string(hmt: usize, rw:&str)->String {
    let mut result= String::new();
    
    for _ in 0..hmt{
        result.push_str(rw);
    }
    result
}


//reverses the digits of a positive integer (advanced) 
fn reversing_digits(mut n : u32)-> u32{
    let mut reversed = 0;

    while n > 0 {
        let digit = n%10;
        reversed = reversed * 10 + digit;
        n = n/10;
    }
    reversed

}

#[derive(Debug)]
enum CalculationError {
    EmptyList,
    InvalidInput,
    CantCompare
}

//finds the minimum and the sum of a collection of numbers. (updated)
fn find_min(list: &[i32]) -> Result<i32, CalculationError> {
    if list.is_empty() {
        return Err(CalculationError::EmptyList);
    }

    let mut min = list[0];

    for &val in list {
        if val < min {
            min = val;
        }
        
    }

    Ok(min)
}

fn find_sum(liste: &[i32]) -> Result<i32, CalculationError> {
    if liste.is_empty() {
        return Err(CalculationError::EmptyList);
    }

    let mut sum = 0;

    for l in liste {
        sum+=l
    }
    Ok(sum)
}



//prints the body mass index category (2nd exercise) using pattern matching. 
fn bmi_match(weight1:f32, height1:f32)-> (f32, String){ // do with enum and result
    let calculation_bmi = weight1 / (height1 * height1);

    let category = match calculation_bmi{
        0.0..=18.4 => " you are anorexic",
        18.5..=24.9 => "you are normal",
        25.0..=29.9 => "you are kinda fat",
        30.0..=34.9 => "you are fat",
        35.0..=44.9 => "GOOD MORNING FATTIES",
        _ => "look at that big chunk"
    };
    println!("Your BMI is {calculation_bmi}");
    (calculation_bmi, category.to_string())
}


//returns the longest prefix of a collection having numbers smaller than n. 
fn prefix(prefix_list: &[i32], z:i32)->  Vec<i32>{

    let mut my_prefix = Vec::new();

    for i in 0..prefix_list.len() {
        if prefix_list[i] < z {
            my_prefix.push(prefix_list[i]);
        } else{
            break;
        }
    }
    my_prefix
}

//returns all prime numbers up to a given number n. 
fn primes_up_to(x: u32) -> Vec<u32> { //copied from bali
    if x < 2 {
        println!("Error: please enter a positive number greater than 1.");
        return Vec::new(); // basic error handling
    }

    let mut primes: Vec<u32> = Vec::new(); // list to store primes

    for i in 2..=x {
        let mut dividable = 0; // counts how many times 'i' divides evenly

        for y in 1..=i {
            if i % y == 0 {
                dividable += 1;
            }
        }

        //this must be inside the outer loop
        if dividable == 2 {
            primes.push(i);
        }
    }

    primes
}



//sorts a collection of people with names and ages by name.
fn sort_ppl(ppl:&mut  Vec<(&str,i32)>){
    let n = ppl.len();

    for k in 0..n{
        for i in 0..n-k-1{
            if ppl[i].0>ppl[i+1].0{
                let temp = ppl[i];
                ppl[i] = ppl[i+1];
                ppl[i+1]= temp;
            }
        }
    }
}


//prints a collection of heroes with names and health points in a nice way by visualizing them below each other with their health points as health bars.
#[derive(Debug)]

struct Hero{
    name: String,
    health: u32
}

impl Hero{
    fn make_bar(&self){
        let mut i = 0;

        while i < self.health {
            print!("█");
            i += 1;
        }
        println!(); 
    }

    fn display(&self){
        print!("{}:" , self.name);
        self.make_bar();
    }
}


//calculate the area and circumference of a graphical object, which can either be a rectangle, a right triangle with sides of equal length, or a circle.

#[derive(Debug)]

    
    struct Rectangle{
        width: u32,
        height: u32,
    }
    impl Rectangle{
        fn visualize(&self){
            let mut i = 0; 
            while i < self.height {
                let mut j = 0; 
                while j < self.width{
                    print!("█");
                    j += 1;
                } 
                println!();
                i +=1;
            }
        }

        fn area(&self)->u32{
            self.height*self.width
        }

        fn circumference(&self)->u32{
            2*(self.height+self.width)
        }
    }



//stepwise build and manipulate a Sudoku board.
//this shit is hard

    
//visualize such a Sudoku board using print statements.
//haha no




//finds the minimum of two values in a tuple.

fn min_of_two_in_tup<T: PartialOrd + Copy>(value: Option<(T, T)>)-> Result<T, CalculationError>{
    
    match value {
        Some((a,b)) => {
            if a==b {
                Err(CalculationError::CantCompare)
            } else if  b < a{
                Ok(b)
            } else {
                Ok(a)
            }
        }
        None => Err(CalculationError::EmptyList)
    }
}

//finds the maximum of a collection of elements having any type.

fn max_of_coll<T: PartialOrd + Copy>(coll: &[T])->  Result<T, CalculationError>{

    if coll.is_empty() {
        return Err(CalculationError::EmptyList);
    }

    let mut max_val = coll[0];

    for i in 1..coll.len(){
        if coll[i] > max_val {
            max_val = coll[i]
        }
    }
 Ok(max_val)
}


//sums up a collection of optional numeric values

fn sum_of_opt_num< T: AddAssign + Copy>(listy: &[Option<T>])-> Option<T>{

    todo!()
    //i couldnt do it.

}


//finds an element in a collection of any type and returns its index.

fn return_index<T: PartialEq>(list: &[T] , target: &T)-> Option<usize>{
    let mut index = 0;
    
    while index < list.len(){
        if &list[index] == target {
            return Some(index);
        }
        index+=1;
    }
    None
}


//sorts a collection of elements having any type (swap method is allowed).

fn sort<T: PartialOrd>(list: &mut [T])-> Result<(), CalculationError> {
   
    if list.is_empty() {
        return Err(CalculationError::EmptyList)
    }

   let len = list.len();

   for i in 0..len{
       for j in 0..len-i-1{
           if list[j] > list[j+1] {
               list.swap(j,j+1)
           }
       }
   }
   Ok(())
}


use std::fmt::Display;
//joins elements of any type into a string using a given delimiter.
fn join_elements<T: Display, U: Display>(array: &[T], delimeter: U) -> Option<String> {
    
    if array.is_empty(){
        return None;
    } 
    
    let mut my_list = String::new();
    let len = array.len();
    
    for i in 0..len {
        my_list.push_str(&format!("{}", array[i]));
        if  len-1 > i {
                my_list.push_str(&format!("{}", delimeter));
        }

    }
    
    
    
    Some(my_list) 
}

#[derive(Debug)]
//1-Write a function that finds the maximum and the sum of a collection of numbers.
enum Error{
    EmptyList
}

fn find_max<T: PartialOrd+ Clone>(list:&[T])-> Result< T, Error>{
    
    if list.is_empty(){
        return Err(Error::EmptyList);
    }
    let mut max_val= list[0].clone();
    
    for i in 1..list.len() {
        if max_val < list[i] {
            max_val = list[i].clone();
        } 
    }
    Ok(max_val)
}
fn find_sum<T:AddAssign+ Clone>(list:&[T])-> Result< T, Error>{
    if list.is_empty(){
        return Err(Error::EmptyList);
    }
    let mut sum = list[0].clone();
    
    for i in 1..list.len(){
        sum += list[i].clone();
    }
    Ok(sum)
}

//2-Write a function that returns all prefixes of a collection (pick any type for the elements within the collection).

fn prefix<T: std::fmt::Debug>(list:&[T])->Vec<&[T]>{
    
    let mut vector = Vec::new();
    for i in 0..list.len(){
        println!("{:?}", &list[0..=i]); 
        vector.push(&list[0..=i]);
    } 
    vector
}
    //3-Write a function that rotates a collection of string literals n times to the left. 
    //Rotating a collection one time to the left means that every element of the collection 
    //is moved one place (index) to the left, where the leftmost element becomes the last element.,
fn rotate<T: Clone>(list:&[T], count:u32)-> Option<Vec<T>>{
    
    if list.is_empty(){
        return None;
    }
    
    let mut new_vec = Vec::new();
    
    for j in 0..list.len(){
    new_vec.push(list[j].clone());
    }
    for s in 0..count{
        let save = new_vec[0].clone(); 
        let last_element = new_vec.len()-1;
    
        for i in 0..last_element
    {new_vec[i] = new_vec[i+1].clone()}
    new_vec[last_element] = save;
    }
    
    Some(new_vec)
}
    //4-Write a function that converts a String of digits into a number. Tolerate whitespaces at the beginning and the end of the String. 


fn main(){
    //calculates the absolute value of a number.
    println!(" absolute value of the number is: {}",abs_val(-10));
    
    //enforces a number to be in a given range.
    println!("{}", enforce(101));

    //determines the maximum of three numbers.
    println!("maximum of three numbers is: {}",max_of_three(5, 10, 15));

    //determines the amount of days for a given month.
    println!("days in a given month is: {}", days_in_month(2));

    //prints the grade for this course based on four test results.
    println!("your grade is: {}", grading(100.0, 100.0, 100.0, 100.0));

    //prints the body mass index category for a given weight and height.
    println!("your bmi is: {}",bmi(65.7, 1.70));

    //calculates the factorial of a given number (advanced) 
    println!("factorial of the number is :{}", factorial(0)); 

    //mutates a number by adding another number n.
    let mut number = 10;
    mut_nuber(&mut number, 5);
    println!("mutated number is: {}" , number);
    
    //concatenates two strings.
    println!("concetenated string is: {}",concat_str("hi".to_string(), "hello".to_string()));

    //swaps the values of two variables.
    let mut a = 10;
    let mut b = 20;
    println!("{a} , {b}");
    swap_val(&mut a, &mut b);
    println!("{a} , {b}");

    //returns a nicely formatted string congratulation for a given birthday.
    println!("{}", birthday(20));


    //repeats a string n times, thus producing a new string (advanced).
    println!("{}", repeat_string(5, "hi"));


    //reverses the digits of a positive integer (advanced) 
    let num = 21344567;
    let rev = reversing_digits(num);
    println!("{num} reversed version is {rev}");


    //finds the minimum and the sum of a collection of numbers. 
    let my_list = vec![1,2,3,4,5];
    match find_min(&my_list) {
        Ok(min)=> println!("this is the min of the list: {min}"),
        Err(e) => println!("Error: {:?}", e),
    }

    match find_sum(&my_list){
        Ok(sum)=> println!("this is the sum of the list: {sum}"),
        Err(e) => println!("Error: {:?}", e),
    }


    //prints the body mass index category (2nd exercise) using pattern matching. 
    let bmi_result = bmi_match(100.0, 1.65);
    println!("{bmi_result:?}");


    //returns the longest prefix of a collection having numbers smaller than n. 
    let my_main_prefix = prefix(&[], 7);
    println!("{my_main_prefix:?}");


    //returns all prime numbers up to a given number n. 
    let primes1: Vec<u32> = primes_up_to(50);
    println!("{:?}", primes1);



    //sorts a collection of people with names and ages by name.
    let mut ppl = vec![ ("ali", 20), ("zeynep", 19), ("veli", 21)];
    println!("before sorting: {ppl:?}");
    sort_ppl(&mut  ppl);
    println!("after sorting: {ppl:?}");



    //prints a collection of heroes with names and health points in a nice way by visualizing them below each other with their health points as health bars.
    let heroes = [
        Hero {name: "this".to_string() , health: 3},
        Hero {name: "one".to_string() , health: 3},
        Hero {name: "is".to_string() , health: 3},
        Hero {name: "for".to_string() , health: 15},
        Hero {name: "you".to_string() , health: 3},
        Hero {name: "bali".to_string() , health: 3},
    ];
    
    let mut i = 0;
    while i< heroes.len(){
        heroes[i].display();
        i+=1;
    }


 
    //calculate the area and circumference of a graphical object, which can either be a rectangle, a right triangle with sides of equal length, or a circle.
    //visualize such a graphical object using print statements.
    
    let my_rectangle = Rectangle{width: 3, height:5};
    println!("rectangle:");
    my_rectangle.visualize();
    println!("Area: {}", my_rectangle.area());
    println!("Circumference: {}", my_rectangle.circumference());



    //finds the minimum of two values in a tuple.
    
    let result1 = min_of_two_in_tup(Some((10, 2)));
    println!("{:?}", result1); 

    let result2: Result<i32, CalculationError> = min_of_two_in_tup(None);
    println!("{:?}", result2);


    //finds the maximum of a collection of elements having any type.
    let sol1 = vec![1,2,3];
    
    match max_of_coll(&sol1) {
        Ok(o)=> println!("max is : {o:?}"),
        Err(e)=> println!("error : {e:?}")
    }

    let sol2= vec!["a", "b"];
    match max_of_coll(&sol2){
        Ok(o)=> println!("max is : {o:?}"),
        Err(e)=> println!("error : {e:?}")
    }

    //sums up a collection of optional numeric values 
    //meh 


    //finds an element in a collection of any type and returns its index.
    
    let numbers = [1,2,3,4,5];
    let result1= return_index(&numbers, &2);
    println!("index: {:?}", result1);

    //sorts a collection of elements having any type (swap method is allowed).
    let mut a = vec![58,68,4,5];
    
    match sort(&mut a) {
        Ok(_)=> println!("sorted : {a:?}"),
        Err(e)=> println!("error : {e:?}")
    }

    let mut a2: Vec<i32>= vec![];
    match sort(&mut a2){
        Ok(_)=> println!("sorted : {a2:?}"),
        Err(e)=> println!("error : {e:?}")
    }


    //joins elements of any type into a string using a given delimiter.
    let my_list = ["1","2","3"];
    let bla = "-";
    println!("{:?}", join_elements(&my_list,bla));


    //1-Write a function that finds the maximum and the sum of a collection of numbers.
    let my_list = [1,2,3,4,5];
    match find_max(&my_list){
        Ok(o)=> println!("this is the max: {o:?}"),
        Err(e)=> println!("an error has occured. : {e:?}"),
    }
    let my_list2 = [1,2,3,4,5];
    match find_sum(&my_list2){
        Ok(o)=> println!("this is the sum: {o:?}"),
        Err(e)=> println!("an error has occured. : {e:?}"),
    }
    //2-Write a function that returns all prefixes of a collection (pick any type for the elements within the collection).
    let vector1 = vec![1,2,3];
    println!("{:?}", prefix(&vector1));
    
    //3-Write a function that rotates a collection of string literals n times to the left. 
    //Rotating a collection one time to the left means that every element of the collection 
    //is moved one place (index) to the left, where the leftmost element becomes the last element.
    let  vector2 = vec![76,45,433,2]; 
    let count = 2;
    println!("{:?}", rotate(&vector2, count));
    
    //4-Write a function that converts a String of digits into a number. Tolerate whitespaces at the beginning and the end of the String. 
     



}