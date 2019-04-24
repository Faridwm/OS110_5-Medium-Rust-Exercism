# ISBN Verifier

ISBN adalah singkatan dari *International Standard Book Number* (Nomor Buku Standar Internasional), ISBN sendiri merupakan nomor unik yang terdiri dari 10/13 digit (dalam kasus ini 10 digit) sebagai pemberi identifikasi terhadap satu judul buku yang diterbitkan.

Format ISBN dengan 10 digit terdiri dari 9 digit (0 - 9) dengan atau tanpa strip(-) di tambah dengan satu karakter cek yang terletak di digit terakhir ISBN, karakter cek ini bisa berupa digit 0 - 9 ataupun X (dimana X sendiri disini mewakili nilai 10).

Untuk memverifikasi ISBN 10 digit dapat menggunakan rumus berikut:
```
(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11
```
Jika hasil dari rumus berikut adalah 0 maka dapat disimpulkan ISBN tersebut benar, begitupun sebaliknya.

## Masalah
Dalam ISBN-Verifier kita diberikan barisan digit, dimana kita akan mengecek barisan digit itu apakah memenuhi kriteria ISBN atau tidak.

# Solusi

## Menghapus Strip dan Mengubah ke dalam list char
Pertama kita hapus jika di input terhadap strip, jika tidak maka tidak terpengaruh
``` rust
let bar = isbn.replace("-", "");
```

lalu mengubah input dari String ke list char 
```rust
let char_bar: Vec<char> = bar.chars().collect(); 
```

## Verifier
Untuk memverifier pertama kali kita cek apakah panjang nya adalah 10 atau tidak.
```rust
if char_bar.len() != 10 { //melakukan pengecekan apakah panjang dari digit == 10 atau tidak, jika tidak maka ISBN salah
        return false;
    }
```

Jika panjang String sebesar 10, maka masuk ke dalam rumus.
```rust
let mut sum = 0;
    for i in 1..=10{
        //is_numeric mengecek apakah setiap digit merupakan angka atau bukan (non-angka hanya X dan itu terletak di digit akhir ISBN)
        if char_bar[i-1].is_numeric() { 
            sum += (11 - i as i32) * (char_bar[i-1].to_string() //mengubah char enjadi String
                                        .parse::<i32>() //mengubah String ke integer 32 bit (Masih dalam Result)
                                        .unwrap()); // me-return panic! dan mengeluarkan hasil parse dari result ke i32  
            //ref: https://stackoverflow.com/questions/43983414/how-to-convert-char-to-integer-so-that-1-becomes-1
        }
        else if char_bar[i-1] == 'X' && i == 10 { //mengecek nilai X == 10 dan berada di digit terakhir ISBN
            sum += 10;
        }
        else {
            return false;
        }
    }
```

### Penjelasan
* Pertama deklarasi sum sebagai varibel hasil akhir dari rumus
* Lalu kita cek apakah setiap char di dalam char list berupa angka atau bukan
* Jika berupa angka maka akan masuk ke rumus 
```rust
if char_bar[i-1].is_numeric() { 
            sum += (11 - i as i32) * (char_bar[i-1].to_string() //mengubah char enjadi String
                                        .parse::<i32>() //mengubah String ke integer 32 bit (Masih dalam Result)
                                        .unwrap()); // me-return panic! dan mengeluarkan hasil parse dari result ke i32
        }
```
* Jika terdapat karakter X yang berada pada di akhir list maka X bernilai 10
```rust
else if char_bar[i-1] == 'X' && i == 10 { //mengecek nilai X == 10 dan berada di digit terakhir ISBN
            sum += 10;
        }
```
* 



