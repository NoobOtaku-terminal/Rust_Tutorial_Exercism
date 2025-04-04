use std::fmt;
pub struct Clock{
    hours: i32,
     minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (mut hrs,mut mins):(i32,i32);
        mins = minutes + hours*60;
        hrs = mins/60;
        hrs = hrs%24;
        if hrs < 0{
            hrs +=24;
        }
        mins = mins%60;
        if mins < 0{
            mins += 60;
        }
        Self{
            hours: hrs,
            minutes: mins,
        }
        
       
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (mut hrs,mut mins):(i32,i32);
        mins = self.hours*60+self.minutes+minutes;
        hrs = mins/60;
        hrs = hrs%24;
        if hrs < 0{
            hrs += 24;
        }
        mins = mins%60;
        if mins < 0{
            mins += 60;
        }
        Self{
            hours: hrs,
            minutes:mins,
        }      
    }
}
impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let mut hours:String = String::new();
        let mut minutes:String = String::new();
        let (hrs,mins):(i32,i32);
        hrs = self.hours;
        mins = self.minutes;
        if hrs < 10{
            let zero : &str = "0";
            hours.push_str(zero);
            hours.push_str(&hrs.to_string());
        }else{
            hours.push_str(&hrs.to_string());
        }
        if mins < 10{
            let zero : &str = "0";
            minutes.push_str(zero);
            minutes.push_str(&mins.to_string());
        }else{
            minutes.push_str(&mins.to_string());
        }
          write!(f, "{}:{}",hours , minutes)
        
    }
}
