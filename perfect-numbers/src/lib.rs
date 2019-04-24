/// https://exercism.io/my/solutions/5c3ce401ee57420a8ca3b1731bbf089b
#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    let jumlah: u64 = (1..num).filter(|x| num % x == 0).sum(); //https://doc.rust-lang.org/std/iter/trait.Iterator.html
    if jumlah == num {
        return Some(Classification::Perfect);
    }
    else if jumlah > num {
        return Some(Classification::Abundant);
    }
    else {
        return Some(Classification::Deficient);
    }
}
