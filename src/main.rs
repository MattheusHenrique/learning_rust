use chrono::{Duration, NaiveTime, Timelike};
//Two days learning Rust. My code is very ugly :(
fn main(){
    let mut time = Clock::new(1, 50);
    time = Clock::add_minutes(&time, 100);
    let time2 = Clock::new(3, 30);

    if time.data == time2.data{
        println!("FOI");
    }



    println!("{}:{}", time.data.hour(), time.data.minute());

}

pub struct Clock{
    data: NaiveTime,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut minutes = minutes;
        let mut hours = hours;
        if minutes >= 59 {
            let new_minutes = minutes%60;
            let new_hours = hours + (minutes/60);
            minutes = new_minutes;
            hours = new_hours;
        }
        let hours_u32 = hours as u32;
        let minutes_u32 = minutes as u32;
        let time = NaiveTime::from_hms(hours_u32, minutes_u32, 0);
        Clock{
            data: time
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minutes_i64 = minutes as i64;
        let time = self.data + Duration::minutes(minutes_i64);
        Clock{
            data: time
        }

    }

}

    

