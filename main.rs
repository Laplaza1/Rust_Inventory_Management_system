use std::io;


fn main() {
    let mut input = String::new();
    let x:String = String::from("Lebron James");

    println!("{}","Who is the Greatest basketball player in the world");
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("{}", input.trim() == x);
        //assert_eq!(input.trim(),x);
        match input.trim().eq(&x.to_string()) {
            true => println!("Correct!!"),
            _ => println!("Not Correct")}
        input.clear();
        }
    }




