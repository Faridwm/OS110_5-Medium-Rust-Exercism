/// Determines whether the supplied string is a valid ISBN number
/// https://exercism.io/my/solutions/a38358d9685947118c69a29e8d8d0ef7
pub fn is_valid_isbn(isbn: &str) -> bool {
    
    let bar = isbn.replace("-", ""); //mengilangkan tanda strip pada input ISBN
    let char_bar: Vec<char> = bar.chars().collect(); //https://stackoverflow.com/questions/47829646/how-do-i-convert-a-string-to-a-list-of-chars

    if char_bar.len() != 10{ //melakukan pengecekan apakah panjang dari digit == 10 atau tidak, jika tidak maka ISBN salah
        return false;
    }
    let mut sum = 0;
    for i in 1..=10{
        //is_numeric mengecek apakah setiap digit merupakan angka atau bukan (non-angka hanya X dan itu terletak di digit akhir ISBN)
        if char_bar[i-1].is_numeric() { 
            sum += (11 - i as i32) * (char_bar[i-1].to_string() //mengubah char enjadi String
                                        .parse::<i32>() //mengubah String ke integer 32 bit (Masih dalam Result)
                                        .unwrap()); // me-return panic! dan mengeluarkan hasil parse dari result ke i32
                                        //https://stackoverflow.com/questions/43983414/how-to-convert-char-to-integer-so-that-1-becomes-1
        }
        else if char_bar[i-1] == 'X' && i == 10 { //mengecek nilai X == 10 dan berada di digit terakhir ISBN
            sum += 10;
        }
        else {
            return false;
        }
    }
    sum % 11 == 0
}
