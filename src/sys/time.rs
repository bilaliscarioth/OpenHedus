use std::time::SystemTime;

pub struct LseTime {
    pub year : String,
    pub month: String,
    pub day: String,
    pub hour : String,
    pub minutes: String,
}


impl LseTime {
    pub fn now()  -> Result<LseTime, throw::Error<std::string::String>>{
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => {
                let s : u64 = n.as_secs();
                let z : i64 = (s as i64)/86400 + 719468;

                let mut year : i64 = z;

                if z < 0 {
                    year -= 146096;
                }

                year /= 146097;

                let doe : u64 = (z-year * 146097) as u64;
                let yoe : u64 = (doe-doe/1460 + doe/36524 - doe/146096) / 365;

                let mut y : i64 =  yoe as i64  + year * 400;
                let doy = doe-(365*yoe + yoe/4 -yoe/100);
                let mp = (5*doy +2)/153;
                let d : u64 =  doy - (153*mp+2)/5 + 1;
                let mut m : i64 = mp as i64;

                if mp < 10{
                    m+=3 ;
                } else {
                    m+=-9;
                }

                y += (m <= 2) as i64;

                let res : i64 = s as i64 - ((y-1970)*31557600)  + (m*2682000) + (d*86400) as i64;

                let mi = (res / 60 ) % 60;
                let h = (res / 3600) %24;

                Ok(LseTime{
                    year: y.to_string(),
                    month: m.to_string(),
                    day: d.to_string(),
                    hour: h.to_string(),
                    minutes: mi.to_string()
                })

            },
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        }
    }
}
