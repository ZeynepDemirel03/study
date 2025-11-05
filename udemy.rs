#[derive(Debug)]
struct TaylorSwiftSong{
    title: String,
    release_year: u32,
    duration_secs: u32
}

impl TaylorSwiftSong{
    fn display_song_info(&self){
        println!("{}, {}, {}", self.title, self.release_year,self.duration_secs)
    }
    fn double_len(&mut self){
        self.duration_secs = self.duration_secs*2;
    
    }
    fn is_longer_than (&self , other: &TaylorSwiftSong)-> bool{
        self.duration_secs > other.duration_secs
    }

}
#[derive(Debug)]

struct Food{
    name: String
}

#[derive(Debug)]
struct Restaurant{
    reservations: u32,
    has_mice_infestation: bool,
}

impl Restaurant {
    fn chef_special(&self) -> Option<Food>{
        if self.has_mice_infestation{
            None
        } else if self.reservations < 12{
            Some(Food {
                name: String::from("uni sashimi")
            })
        } else {
            Some(Food{
                name: String::from("strip steak")
            }
            )
        }
    }

    fn deliver_burger(&self , address: &str)-> Result<Food, String>{
        if self.has_mice_infestation{
            Err(String::from("Sorry, we have a mice problem"))
        } else if address.is_empty(){
            Err(String::from("No delivery address specified"))
        } else{
            Ok(Food{name:String::from("burger")})
        }
    } 
}

fn main(){
    let mut song = TaylorSwiftSong {
        title: String::from("actually romantic"),
        release_year: 2025,
        duration_secs: 230 
    };
    song.display_song_info();
    song.double_len();
    song.display_song_info();

    //burger example 
    let r1 = Restaurant{
        reservations:11,
        has_mice_infestation:true
    };

    println!("{:?}", r1.chef_special());
    println!("{:?}", r1.deliver_burger("123 elm street"));

    let r2 = Restaurant{
        reservations:15,
        has_mice_infestation:false
    };
    println!("{:?}", r2.chef_special());
    println!("{:?}", r2.deliver_burger(""));
    println!("{:?}", r2.deliver_burger("456 Oak Avenue"))
}   