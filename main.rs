use std::io;
use serde_json::{from_str, to_string};
use serde::{Deserialize, Serialize};
use serde::de::Unexpected::Str;

#[derive(Deserialize, Serialize,Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: usize,
    id:usize,
    title: String,
    completed:bool,
}


fn main() {
    check_json(1.to_string())




}

fn retrieve_json(){
    let json = std::fs::read_to_string("C:/Users/L1K3A/RustroverProjects/Rust_Inventory_check/src/todos.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json);
    println!("{:#?}",todos);


}

fn check_json(input:String){
    let json = std::fs::read_to_string("src/todos.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json).unwrap();

    for item in todos {
        if item.user_id.to_string().eq(&input) {println!("{:#?}", item)}
        else{if item.id.to_string().eq(&input) {println!("{:#?}", item)}
        else {if item.title.to_string().eq(&input) {println!("{:#?}", item)}
        else {if item.completed.to_string().eq(&input) {println!("{:#?}", item) }}}}


    }



}



fn input_loop() {
    let mut input = String::new();
    let x: String = String::from("Lebron James");

    println!("{}", "Who is the Greatest basketball player in the world");
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("{}", input.trim() == x);
        //assert_eq!(input.trim(),x);
        match input.trim().eq(&x.to_string()) {
            true => println!("Correct!!"),
            _ => println!("Not Correct")
        }
        input.clear();
    }
}







