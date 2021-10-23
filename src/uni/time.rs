use chrono::prelude::*;

pub struct Time{
    pub local: bool
}

impl Time{
    pub fn now() -> DateTime<Local> {
        let now = Local::now();
        now
    }
    
    pub fn utc_time() -> DateTime<Utc> {
        Utc::now()
    }
}