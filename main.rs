use io::stdin;
use path::absolute;
use std::fmt::{Binary, Debug};
use std::{io, path, str};
use std::process::{exit};
use serde_json::{from_str, to_string_pretty};
use serde::{Deserialize, Serialize, Serializer};
use std::fs::{File, OpenOptions};
use std::io::{Read,Write};
use serde::__private::de::{IdentifierDeserializer};
use serde::de::IntoDeserializer;
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

            "no" => {println!("Do you want to add to inventory? yes or no");input.clear() ;
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

            "adjust"=> {println!("Please enter what value you want to replace excluding ID because It must remain immutable");input.clear();
                 stdin().read_line(&mut input).expect("Failed to read line");

                match input.trim() {

                  //Creates the user_id var
                "user_id" =>    {
                    println!("Please type the old value you want to adjust");
                    let _key_ = input.trim();
                    let mut _old_val :String = String::new();
                    stdin().read_line(&mut _old_val).expect("Failed to read line");
                    println!("Please type the new value you want to adjust");
                    let mut _new_val :String = String::new();
                    stdin().read_line(&mut _new_val).expect("Failed to read line");
                    adjust_inventory(_key_.to_string(), _old_val, _new_val);break},


                  //Creates the title var
                "title" => {

                    println!("Please type the title");
                    println!("Please type the old value you want to adjust");
                    let mut _key_: &str = &*input.trim();
                    let mut _old_val :String = String::new();
                    stdin().read_line(&mut _old_val).expect("Failed to read line");
                    println!("Please type the new value you want to adjust");
                    let mut _new_val :String = String::new();
                    stdin().read_line(&mut _new_val).expect("Failed to read line");
                    adjust_inventory(_key_.trim().to_string(), _old_val.trim().to_string(), _new_val.trim().to_string());break},

                _ => {
                    println!("{}","not correct please enter the correct fields");
                    break
                },


                }}




            _ => {println!("{}",question);input.clear()}
            }



        }

    }





fn add_to_inventory(_user_id:&str, _id:&str, _title:&str, _completed:&str) {
    let pathbuffer = path::PathBuf::from("src/todos.json");
    let path:String = absolute(pathbuffer).as_deref().unwrap().to_str().unwrap().to_string();

    println!("{}{}{}{}",_user_id.to_string(),_id,_title,_completed);

    let book:Todo = Todo{user_id: _user_id.parse::<usize>().unwrap(),id: _id.parse::<usize>().unwrap(),title: String::from(_title),completed: _completed.eq("true") };
    let mut m =OpenOptions::new()
        .write(true)
        .append(true)
        .open(&*path).unwrap();
    let serialized =to_string_pretty(&book)
        .expect("Failed to serialize JSON");
    let comma:String = ",".to_string();
    let eof:String = "]".to_string();
    let neo_serialized:String = comma+ &*serialized + &*eof;
    let mut file = File::open(&*path).unwrap();
    let mut contents =String::new();
    file.read_to_string(&mut contents).unwrap();
    if let Some(pos) = contents.rfind(']'){contents.remove(pos);
    }
    let mut file = File::create(&*path).unwrap();
    file.write_all(contents.as_bytes()).expect("TODO: panic message");
    m.write_all(neo_serialized.as_bytes()).expect("TODO: panic message");







}

fn adjust_inventory(_key_:String, old_val:String, mut new_val:String) {
    let json = std::fs::read_to_string("src/todos.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json).unwrap();
    //let path: &str = "C:/Users/L1K3A/RustroverProjects/Rust_Inventory_check/src/todos.json";
    let pathbuffer = path::PathBuf::from("src/todos.json");
    let path:String = absolute(pathbuffer).as_deref().unwrap().to_str().unwrap().to_string();
    let mut m = OpenOptions::new()
        .write(true)
        .append(false)
        .open(&*path).unwrap();
    let mut file = File::open(&*path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut counter: usize = 0;
    println!("{old_val}");


    for mut item in todos {
        println!("{counter}");
        println!("old val {}\ncomparison {}\nitem user id val {}", old_val.trim(), item.user_id.to_string().eq(&old_val.trim()), item.user_id.to_string());
        println!("*********************************************");

        match _key_.as_str() {
            "user_id" => if item.user_id.to_string().eq(&old_val.trim())
            {
                println!("{:#?}", item);
                file.read_to_string(&mut contents).unwrap();
                let mut todoer = from_str::<Vec<Todo>>(&json).unwrap();
                let new_new_val: String = new_val.to_string();
                println!("{new_new_val}");
                todoer[counter].user_id = (*new_new_val).trim().parse().unwrap();
                println!("{:?}", todoer[counter].user_id);
                let trial: String = to_string_pretty(&todoer).unwrap();
                m.write_all(trial.as_bytes()).expect("TODO: panic message");
            },

            "id" => if item.id.to_string().eq(&old_val.trim())
            {
                println!("{:#?}", item);
                file.read_to_string(&mut contents).unwrap();
                let mut todoer = from_str::<Vec<Todo>>(&json).unwrap();
                todoer[counter].id = new_val.trim().parse().unwrap();
                println!("{:?}", todoer[counter].id);
                let trial: String = to_string_pretty(&todoer).unwrap();
                m.write_all(trial.as_bytes()).expect("TODO: panic message");
            }

            "title" => if item.title.to_string().eq(&old_val.trim())
            {
                println!("{:#?}", item);
                file.read_to_string(&mut contents).unwrap();
                let mut todoer = from_str::<Vec<Todo>>(&json).unwrap();
                todoer[counter].title = new_val.parse().unwrap();
                println!("{:?}", todoer[counter].title);
                let trial: String = to_string_pretty(&todoer).unwrap();
                m.write_all(trial.as_bytes()).expect("TODO: panic message");
            },

            "completed" => if item.completed.to_string().eq(&old_val.trim())
            {
                println!("{:#?}", item);
                file.read_to_string(&mut contents).unwrap();
                let mut todoer = from_str::<Vec<Todo>>(&json).unwrap();
                todoer[counter].completed = new_val.parse().unwrap();
                println!("{:?}", todoer[counter].completed);
                let trial: String = to_string_pretty(&todoer).unwrap();
                m.write_all(trial.as_bytes()).expect("TODO: panic message");
            },

            _ => if item.user_id.to_string().eq(&old_val)
            {
                println!("{:#?}", item);
            },
        }
        counter += 1;
    }


    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
}


fn check_len_of_item_json() {
    let json = std::fs::read_to_string("src/todos.json").unwrap();
    let todos = from_str::<Vec<Todo>>(&json).unwrap();
    let mut item_count:usize = 0;

    for item in todos {

        println!("This is the index {item_count} of the Todo variable",);
        println!("Key {}---Value {}", "userId".to_string().len(),item.user_id.to_string().len() );
        println!("Key {}---Value {}", "Id".to_string().len(), item.id.to_string().len());
        println!("Key {}---Value {}", "title".to_string().len(), item.title.to_string().len());
        println!("Key {}---Value {}", "completed".to_string().len(), item.completed.to_string().len());
        item_count += 1;

    }
}
fn check_old_val_pos(old_val:&str){
    let pathbuffer = path::PathBuf::from("src/todos.json");
    let path:String = absolute(pathbuffer).as_deref().unwrap().to_str().unwrap().to_string();
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let user_id__: Option<usize> = contents.rfind(&*old_val);
    println!("{}",user_id__.unwrap());
}
