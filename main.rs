use io::stdin;
use std::collections::HashMap;

use std::fmt::{Binary, Debug};
use std::io;
use std::process::{exit};
use serde_json::{from_str};
use serde::{Deserialize, Serialize, Serializer};

use serde_json::Value;

use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::ser::SerializeStruct;


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
   //add_to_inventory()





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
        stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("\n");
        
        //checks if initial input is yes or no or quit or resets if not a valid answer
        match input.trim(){
            "yes" =>{input.clear();println!("{}","Please type the Value you want to check; It is case sensitive.");
                stdin().read_line(&mut input)
                .expect("Failed to read line");
                check_json(input.trim().to_string());
                println!("{}",question)
            } ,

            "no" => {println!("Do you want to adjust inventory? yes or no");input.clear() ;
                stdin().read_line(&mut input).expect("Failed to read line");

                if input.trim().eq("yes") {
                    input.clear();

                    //Creates the User id var
                    println!("Please type the User Id");
                    let mut user_id_ :String = String::new();
                    stdin().read_line(&mut user_id_).expect("Failed to read line");

                     //Creates the id var
                    println!("Please type the Id");
                    let mut id_ :String = String::new();
                    stdin().read_line(&mut id_).expect("Failed to read line");

                     //Creates the title var
                    println!("Please type the title");
                    let mut title_ :String = String::new();
                    stdin().read_line(&mut title_).expect("Failed to read line");

                     //Creates the completed var
                    println!("Please type if its completed or not in true or false");
                    let mut completed :String = String::new();
                    stdin().read_line(&mut completed).expect("Failed to read line");

                     //passes all the created vars into the add_to_inventory func along with some trimming
                    add_to_inventory(&*user_id_.trim_ascii(),&*id_.trim_ascii(),&*title_.trim_ascii(),&*completed.trim_ascii());
                    println!("Successfully added, Please hit enter to continue!");}
                else { println!("not yes") };


                }
            "quit"=> {println!("{}","Now quitting!");exit(0);},
            _ => {println!("{}",question);input.clear()}
            }



        }

    }





fn add_to_inventory(_user_id:&str, _id:&str, _title:&str, _completed:&str) {
    let path = "C:/Users/L1K3A/RustroverProjects/Rust_Inventory_check/src/todos.json";
    let json = std::fs::read_to_string("src/todos.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json).unwrap();

    println!("{}{}{}{}",_user_id.to_string(),_id,_title,_completed);

    let book:Todo = Todo{user_id: _user_id.parse::<usize>().unwrap(),id: _id.parse::<usize>().unwrap(),title: String::from(_title),completed: _completed.eq("true") };
    let mut m =OpenOptions::new()
        .write(true)
        .append(true)
        .open(path).unwrap();
    let serialized = serde_json::to_string_pretty(&book)
        .expect("Failed to serialize JSON");
    let comma:String = ",".to_string();
    let eof:String = "]".to_string();
    let neo_serialized:String = comma+ &*serialized + &*eof;
    let mut file = File::open(path).unwrap();
    let mut contents =String::new();
    file.read_to_string(&mut contents).unwrap();
    if let Some(pos) = contents.rfind(']'){contents.remove(pos);
    }
    let mut file = File::create(path).unwrap();
    file.write_all(contents.as_bytes()).expect("TODO: panic message");
    m.write_all(neo_serialized.as_bytes()).expect("TODO: panic message");







}

fn adjust_inventory(){
    let mut todo: HashMap<String,Value> = serde_json::from_str(r#"{"id": 2}"#).expect("unable to parse JSON");

    if let Some(mut id) = todo["id"].as_u64(){id +=1;
    todo.insert("id".to_string(),Value::from(id));
    }else { panic!("Id is missing or not a number");
    }

    println!(
        "{}",serde_json::to_string(&todo).expect("unable to serialize"));





}




