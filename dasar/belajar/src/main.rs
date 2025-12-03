fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test(){
    println!("Hello Test");
    println!("first time writing rust code!!");
}


/***
 * Variable adalah wadah untuk menyimpan suatu value
 * Variable memiliki 2 tipe, yaitu immutable dan mutable
 */
#[test]
fn test_var(){
    // immutable variable or Constant var
    let name = "ZUlfan Taqiyudin";
    println!("Hello {}", name);

    // mutable var
    let mut city = "Bogor";
    println!("Tinggal di kota {}", city);

    city = "Jakarta";
    println!("Kerja di {}", city);
}


/*** 
 * Shadowing adalah saat kita membuat variable dengan menggunakan nama yang sama.
 * hal tersebut bisa dilakukan tapi akan menutup variable yang lamanya.
 */
#[test]
fn test_shadowing(){

    // create variable with name name;
    let name = "Zulfan Taqiyudin";
    println!("Hello {}", name);

    // shadowing
    let name = 20;
    println!("My age is: {}", name);
}


/***
 * Tipe data dalam pemrograman Rust dibagi menjadi dua bagian:
 * 1. Scalar Type
 *      => Scalar type adalah tipe data yang bernilai tunggal (single value)
 *      Seperti Integer, Float, Boolean, Char (untuk char kita menggunakan petik satu "''"")
 * 2. Compound Type
 *      => Compound Type adalah tipe data yang mempresentasikan beberapa value (>= 1) dalam satu tipe
 *      Seperti tuple dan array
 *          o> Tuple = kumpulan beberapa data yang bisa berbeda tipe data
 *          o> Array = kumpulan beberapa data yang harus memiliki tipe data yang sama
 * 
 *  ===== HOW TO DECLARE DATA TYPE ===== 
 *  - Implicit Type
 *      => Tidak mendeklarasikan tipe data, karena Rust bisa otomatis mendeteksi tipe data apa yang kita assign ke dalam sebuah variable
 *  - Explicit Type
 *      => Mendeklarasikan tipe data secara jelas
 *      Cara mendeklarasikan sebuah tipe data yaitu dengan menggunakan tanda ":"(titik dua)
 */
#[test]
fn test_data_type(){
    // explicit data type
    let age: i32 = 20; //declare Integer
    println!("Umur kita adalah {}", age);

    let height: f64 = 169.5; //declare floating point
    println!("Tinggi kita adalah {}", height);
}


/***
 * DATA TYPE CONVERTION (NUMBER)
 *  Untuk melakukan conversi data type, kita bisa menggunakan kata kunci "as";
 *  Data type conversion bisa dilakukan untuk nilai data yang lebih kecil ke yang lebih besar dan sebaliknya.
 *  Tetapi saat melakukan conversi data type dari type data yang lebih besar ke lebih kecil kita harus berhati hati karena bisa jadi *  data tersebut tidak bisa ditampung oleh tipe data yang akan dikonversi.
 *  Hal tersebut dinamakan Integer Overflow (Nilai data lebih besar dibanding data tipenya)
 */
#[test]
fn test_conversion_data_type_number(){
    let a: i8 = 10;
    println!("{}", a);
    
    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 10000000000;
    let e: i8 = d as i8; //konversi bisa dilakukan tapi output yang diinginkan akan aneh. disebabkan Integer Overflow
    println!("{}", e);
}


/***
 * melakukan operasi matematika menggunakan nilai dari variable sendiri
 */
#[test]
fn test_augmented_assignment(){

    let mut a = 10;

    a+=12; // a = a + 12;
    println!("{}", a);
    a-=12; // a = a - 12;
    println!("{}", a);
    a*=12; // a = a * 12;
    println!("{}", a);
    a/=12; // a = a / 12;
    println!("{}", a);
    a%=12; // a = a / 12
    println!("{}", a);

}


#[test]
fn test_boolean_comparison(){
    let absen = 80;
    let nilai_akhir = 93;

    let lulus_absen = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 80;

    let lulus: bool = lulus_absen && lulus_nilai_akhir;
    println!("{}", lulus)
}

#[test]
fn test_char_type(){
    let char1: char = 'a';
    let char2 = 'b';

    println!("{} {}", char1, char2);
}


/** COMPOUND DATA TYPES */

// TUPLE
/***
 * Tuple => kumpulan lebih dari satu tipe data
 * Jumlah tuple final, tidak bisa berkurang ataupun bertambah
 * Secara default tuple merupakan IMMUTABLE, 
 * tapi jika kita mendeklarasikan variable tuple menggunakan "mut" maka tuple tersebut akan berubah menjadi MUTABLE
 * To make a tuple we use "()";
 * 
 * Untuk mengakses data tuple kita menggunakan "." (titik), lalu di ikuti dengan nomor index
 * Index tuple start from 0;
 */
#[test]
fn test_tuple(){

    // declaring tuple
    let data = (1, "a", true);
    let data2: (i32, f64, &str) = (23, 23.5, "23.5");

    println!("{:?}", data);
    println!("{:?}", data2);

    // access data tuple per index
    println!("umur saya: {}, berat badan saya: {}", data2.0, data2.1);

    // desctructuring tuple
    let (a, b, c) = data;
    println!("{} {} {}", a, b, c );


    // mutable tuple
    let mut mut_tuple: (i32, bool, char) = (2, false, 'a');

    // bisa
    mut_tuple.0 += 3;
    // error
    // data.0 +=1;

}

fn unit(){
    println!("make some unit")
}

#[test]
fn test_unit() {
    let result = unit();

    println!("{:?}", result);    
}