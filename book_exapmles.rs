#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}
#[derive(Debug)]
enum Mood {
    Happy,
    Sleepy,
    Angry,
}

#[derive(Debug)]
struct Animal {
    age: u8,
    animal_type: AnimalType,
    mood: Mood,
}

impl Animal {
    fn show_mmod(&self){
        match self.mood{
            Mood::Happy=> println!("This animal is happy"),
            Mood::Sleepy=> println!("This animal is sleepy"),
            Mood::Angry => println!("This animal is angry"),
        }
    }
    fn new_cat() -> Self {
        Self { 
            age: 10, 
            animal_type: AnimalType::Cat,
            mood: Mood::Happy,
        }
    }
    fn new_dog() -> Self{ 
        Self{
            age: 9,
            animal_type: AnimalType::Dog,
            mood: Mood::Happy,

        }
    }
    fn new_with_age(age: u8 , animal_type: AnimalType) -> Self{
        Self {
            age,
            animal_type,
            mood: Mood::Sleepy,
        }   
    }
    /*fn check_type(&self) {
        match self.animal_type {
            AnimalType::Dog => println!("The animal is a dog"),
            AnimalType::Cat => println!("The animal is a cat"),
        }
    }
    */
    fn check_type(&self) {
    let Animal { animal_type, .. } = self;
    match animal_type {
        AnimalType::Dog => println!("The animal is a dog"),
        AnimalType::Cat => println!("The animal is a cat"),
    }
}

 
    fn change_to_dog(&mut self) {
        self.animal_type = AnimalType::Dog;
        println!("Changed animal to dog! Now it's {self:?}");
    }
    fn change_to_cat(&mut self) {
        self.animal_type = AnimalType::Cat;
        println!("Changed animal to cat! Now it's {self:?}");
    }
}


fn main (){
    let mut cat = Animal::new_with_age(3, AnimalType::Cat);
    let  mut dog = Animal::new_with_age(7, AnimalType::Dog);
    cat.check_type();
    cat.show_mmod();
    println!("this is an {:?}" , cat)
}