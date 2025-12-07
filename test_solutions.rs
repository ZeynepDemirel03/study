/*Write a function that finds the maximum and the sum of a collection of numbers.
 Write a function that returns all prefixes of a collection (pick any type for the elements within the collection).
 Write a function that rotates a collection of string literals n times to the left. 
 Rotating a collection one time to the left means that every element of the collection is moved one place (index) to the left,
 where the leftmost element becomes the last element.
 Write a function that converts a String of digits into a number. Tolerate whitespaces at the beginning and the end of the String.
 */
/*Reminder: Define a custom type that represents a generic collection which can either be an owned vector 
or an owned array with a size of 10.
Implement your function signature that finds the maximum of a collection defined by your type.
Implement your function signature which rotates the collection defined by your type n times to the right.
Implement your function signature that returns all prefixes of a collection defined by your type.
Write a function that joins the elements of a collection defined by your type into a string using a passed delimiter.
Implement all of the above as methods for your type instead of functions. */

use std::fmt::Display;


fn main(){
    let liste = [1,2,3,4,5];

    println!("{:?}",max(&liste));

    println!("{:?}",sum(&liste));

    println!("{:?}", prefix1(&liste));

    let dizi = ["a","b","c","d"];

    println!("{:?}", rotates(&dizi,1));

    println!("{}", converts("".to_string()));

    let mut c1 = MyCollection::VecVariant(vec![1,2,3,4,5]);
    let mut c2 = MyCollection::ArrayVariant([10,20,30,40,50,60,70,80,90,100]);
    println!("{:?}", c1.find_max());
    println!("{:?}", c2.find_max());

    c1.rotate_left(1); 
    println!("{:?}", c1); 

    c2.rotate_left(1);
    println!("{:?}", c2);

    let prefixes = c1.prefix2(); // veya prefixes()

    for pref in prefixes {
        println!("{:?}", pref);
    }

    let prefixes = c2.prefix2(); // veya prefixes()

    for pref in prefixes {
        println!("{:?}", pref);
    }

    let chars = MyCollection::VecVariant(vec!["1","2","3"]);
    let cd = chars.joins("-");
    println!("{:?}", cd)

}

fn max(list:&[i32])->(i32, &str){

    let len = list.len();   
    let mut max = list[0];

    if len == 0 {
        return(0, "list is empty"); 
    }

    for i in 0..len{
        if max < list[i] {
            max = list[i]
        }
    }
    (max, "is the maximum value")
}

fn sum(list:&[i32])->(i32,&str){

    let len = list.len();
    let mut summy = 0;
    if len == 0{
        return(0, "list is empty");
    }

    for i in 0..len{
        summy += list[i]
    }
    (summy, "is the sum")

}

fn prefix1(list:&[i32])->Vec<Vec<i32>>{


    let len = list.len();
    let mut result = Vec::new();
    if list.is_empty(){
        return result;
    }

    for i in 0..=len{
        let mut pref = Vec::new();
        for j in 0..i{
            pref.push(list[j]);
        }
        result.push(pref);
    }
    result
}

fn rotates(list:&[&str],n:usize)->Vec<String>{

    let len = list.len();
    let mut result = Vec::new();

    if list.is_empty(){
        return result;
    }

    let shift = n % len;

    let mut i = shift;
    while i < len {
        result.push(list[i].to_string());
        i +=1;
    }

    let mut j = 0;

    while j < shift {
        result.push(list[j].to_string());
        j +=1;
    }

    result

}

fn converts(a:String)->usize{

    let mut number = 0;

    for ch in a.chars(){

        if ch == ' '{
            continue;
        }
        
        if ch >= '0' && ch <= '9' {
            let digit = (ch as u8 - b'0') as usize;
            number = number *10 +digit;
        }
    }
    number
}
#[derive(Debug)]
enum MyCollection<T> {
    VecVariant(Vec<T>),
    ArrayVariant([T;10])
}

impl <T: PartialOrd+Copy+Clone+Display> MyCollection<T> {
    fn find_max(&self)-> Option<T>{
        match self{
            MyCollection::VecVariant(v)=> {
                if v.is_empty(){
                    return None;
                }

                let mut max = v[0];
                for i in 1..v.len(){
                    if max < v[i] {
                        max = v[i]
                    }
                }
                Some(max)
            }

            MyCollection::ArrayVariant(a)=>{
                if a.is_empty(){
                    return None;
                }

                let mut max = a[0];
                for i in 1..a.len(){
                    if max < a[i] {
                        max = a[i]
                    }
                }
                Some(max)
            }
        }
    }

    fn rotate_left(&mut self, n:usize){
        match self{
            MyCollection::VecVariant(v) => {
                let len = v.len();
                if v.is_empty(){
                    return;
                }
                let k = n % len;
                
                for _ in 0..k{
                    let first = v[0];
                    for i in 0..len-1{
                        v[i] = v[i+1]
                    }
                    v[len-1] = first;
                }
            } 
            MyCollection::ArrayVariant(a)=>{
                let len = a.len();
                if a.is_empty(){
                    return;
                }
                let k = n % len;
                
                for _ in 0..k{
                    let first = a[0];
                    for i in 0..len-1{
                        a[i]=a[i+1]
                    }
                    a[len-1] = first;
                }
            }
        }
    }
    fn prefix2(&self)->Vec<Vec<T>>{
        let mut result = Vec::new();
        match self {
            MyCollection::VecVariant(v)=> {
                let len = v.len();

                for i in 1..=len{
                    let mut pref = Vec::new();

                    for j in 0..i{
                        pref.push(v[j]);
                    }
                    result.push(pref);
                }
                result
            }

            MyCollection::ArrayVariant(a)=> {
                let len = a.len();

                for i in 1..=len{
                    let mut pref = Vec::new();

                    for j in 0..i{
                        pref.push(a[j]);
                    }
                    result.push(pref);
                }
                result
            }
        }
    }

    fn joins(&self, delimiter: &str) -> String {
        let mut output = String::new();

        match self {
            MyCollection::VecVariant(v) => {
                let mut i = 0;
                while i < v.len() {
                    output.push_str(&v[i].to_string());
                    if i < v.len() - 1 {
                        output.push_str(delimiter);
                    }
                    i += 1;
                }
            }
            MyCollection::ArrayVariant(a) => {
                let mut i = 0;
                while i < a.len() {
                    output.push_str(&a[i].to_string());
                    if i < a.len() - 1 {
                        output.push_str(delimiter);
                    }
                    i += 1;
                }
            }
        }
        output
    }
}
