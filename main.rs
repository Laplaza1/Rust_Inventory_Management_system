use std::io;
use std::process::exit;
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
    input_loop("Do you want to check Inventory?")





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



fn input_loop(question:&str) {
    let mut input = String::new();


    println!("{} or type quit to terminate program", question );
    loop {
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("\n");
        //assert_eq!(input.trim(),x);
        match input.trim(){
            "yes" =>{input.clear();println!("{}","Please type the Value you want to check; It is case sensitive.");io::stdin().read_line(&mut input)
                .expect("Failed to read line");check_json(input.trim().to_string());println!("{}",question)} ,
            "no" => println!("Do you want to adjust inventory?"),
            "quit"=> {println!("{}","Now quitting!");exit(0);},
            _ => {println!("{}",question)}
        }
        input.clear();
    }
}







