/*Write a function that finds the maximum and the sum of a collection of numbers.
 Write a function that returns all prefixes of a collection (pick any type for the elements within the collection).
 Write a function that rotates a collection of string literals n times to the left. 
 Rotating a collection one time to the left means that every element of the collection is moved one place (index) to the left,
 where the leftmost element becomes the last element.
 Write a function that converts a String of digits into a number. Tolerate whitespaces at the beginning and the end of the String.
 */
/*Reminder: Define a custom type that represents a generic collection which can either be an owned vector or an owned array with a size of 10.
Implement your function signature that finds the maximum of a collection defined by your type.
Implement your function signature which rotates the collection defined by your type n times to the right.
Implement your function signature that returns all prefixes of a collection defined by your type.
Write a function that joins the elements of a collection defined by your type into a string using a passed delimiter.
Implement all of the above as methods for your type instead of functions. */

fn main(){
    let liste = [1,2,3,4,5];

    println!("{:?}",max(&liste));

    println!("{:?}",sum(&liste));

    println!("{:?}", prefix(&liste));

    let dizi = ["a","b","c","d"];

    println!("{:?}", rotates(&dizi,1));
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

fn prefix(list:&[i32])->Vec<Vec<i32>>{


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

fn converts(a:String)->i32{
todo!();
}
