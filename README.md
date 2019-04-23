# OS110_5-Medium-Rust-Exercism

## ISBN Verifier

ISBN adalah singkatan dari *International Standard Book Number* (Nomor Buku Standar Internasional), ISBN sendiri merupakan nomor unik yang terdiri dari 10/13 digit (dalam kasus ini 10 digit) sebagai pemberi identifikasi terhadap satu judul buku yang diterbitkan.

Format ISBN dengan 10 digit terdiri dari 9 digit (0 - 9) di tambah dengan satu karakter cek yang terletak di digit terakhir ISBN, karakter cek ini bisa berupa digit 0 - 9 ataupun X (dimana X sendiri disini mewakili nilai 10).

Untuk memverifikasi ISBN 10 digit dapat menggunakan rumus berikut:
```
(x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5 + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11
```
Jika hasil dari rumus berikut adalah 0 maka dapat disimpulkan ISBN tersebut benar, begitupun sebaliknya.
