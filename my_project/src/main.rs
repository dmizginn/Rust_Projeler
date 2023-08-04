fn main() {
    let message = "Hello, world!";
    let x : i32 = 42;
    let pi: f64 = 3.14;
    let is_rust_fun : bool = true;
    let letter_a : char ='a';

    fn add(x : i32, y: i32) -> i32{
        x+y
    }

    let x = 4;

    if x>=0{
        println!("x negatif değildir.");
    }else{
        println!("x negatif")
    }

    let mut i = 1;

    while i <= 5 {
        println!("{}",i);
        i+=1;
    }
}
