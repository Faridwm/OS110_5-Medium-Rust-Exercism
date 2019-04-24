/// https://exercism.io/my/solutions/0a39bc6dc6a945eabc9ce59f4ade51d6
use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    if candidate == "" {
        return true;
    }
    let mut kata: HashSet<char> = HashSet::with_capacity(candidate.len()); //ref: https://doc.rust-lang.org/std/collections/struct.HashSet.html
    let char_kata: Vec<char> = candidate.chars().collect();
    let mut char_nodup: Vec<char> = Vec::new(); //untuk membuat list char dengamengilangkan karakter selain huruf
    //memasukkan ke dalam hashset
    for i in char_kata.clone(){
        if i.is_alphabetic(){
            kata.insert(i.to_ascii_uppercase());
        }
    }

    //memasukkan ke dalam list char dengan mengilangkan karakter non-huruf
    for c in char_kata.clone(){
        if c.is_alphabetic(){
            char_nodup.push(c);
        }
    }

    kata.len() == char_nodup.len()
}
