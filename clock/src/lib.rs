/// https://exercism.io/my/solutions/97192f1985b240e898e8ba0bf00b9106
use::std::fmt;

pub struct Clock{
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::ubah_waktu(hours, minutes) // https://users.rust-lang.org/t/calling-impl-functions-from-another-impl-function/2086/3
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::ubah_waktu(self.hours, self.minutes + minutes)
    }

    pub fn ubah_waktu(hours: i32, minutes: i32) -> Self {
        let mut waktu = (hours * 60) + minutes;

        //masalah jika hours / minutes bernilai negatif
        let wkatu_sehari = 60 * 24;
        if waktu < 0 {
            waktu = (waktu % wkatu_sehari) + wkatu_sehari;
        }
        else {
            waktu = waktu % wkatu_sehari;
        }
        
        let menit_benar = waktu % 60;
        let jam_benar = (waktu / 60) % 24;

        Clock {
            hours: jam_benar,
            minutes: menit_benar
        }

    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Clock {{ x: {:02}, y: {:02} }}", self.hours, self.minutes)
    }
}

//https://doc.rust-lang.org/std/cmp/trait.PartialEq.html
impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        (self.hours == other.hours) && (self.minutes == other.minutes)
    }
}


