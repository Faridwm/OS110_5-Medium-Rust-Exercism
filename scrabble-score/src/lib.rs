/// Compute the Scrabble score for a word.
/// https://exercism.io/my/solutions/32ce0cd5d0c64c9db4a9f9c86ff57166
pub fn score(word: &str) -> u64 {
    let kata = word.to_lowercase(); //untuk membuat input menjadi huruf besar semua
    let mut skor = 0; //hasil akhir skor scrabble
    let char_bar: Vec<char> = kata.chars().collect(); //memasukkan kata ke dalam list char
    for i in char_bar {
        match i { //https://doc.rust-lang.org/rust-by-example/flow_control/match.html
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => skor += 1,
            'd' | 'g' => skor += 2,
            'b' | 'c' | 'm'| 'p' => skor += 3,
            'f' | 'h' | 'v' | 'w' | 'y' => skor += 4,
            'k' => skor += 5,
            'j' | 'x' => skor += 8,
            'q' | 'z' => skor += 10,
            _ => skor += 0,
        }
    }
    skor
}