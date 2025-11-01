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

fn main(){
    let mut song = TaylorSwiftSong {
        title: String::from("actually romantic"),
        release_year: 2025,
        duration_secs: 230 
    };
    song.display_song_info();
    song.double_len();
    song.display_song_info();
}   
