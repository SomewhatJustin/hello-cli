use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    #[serde(rename = "userId")]
    user_id: i32,
    id: i32,
    title: String,
    completed: bool,
}

#[tokio::main]
async fn main() {
    println!("Give me a string and I will reverse it oooooohhhh");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("fail!");

    let reversed_string = reverse_string(&user_input);
    println!("{reversed_string}");

    println!("Next: let's get some text from the internet");
    let internet_todo = fetch_todo("https://jsonplaceholder.typicode.com/todos/1")
        .await
        .unwrap();

    println!("todo: {:?}", internet_todo)
}

fn reverse_string(string: &str) -> String {
    let mut reversed_string = String::new();
    for c in string.chars().rev() {
        reversed_string.push(c);
    }
    String::from(reversed_string)
}

async fn fetch_todo(url: &str) -> Result<Todo, reqwest::Error> {
    let body: String = reqwest::get(url).await?.text().await?;

    println!("text is {body}");

    let deserialized: Todo = serde_json::from_str(&body).expect("Failed to deserialize");

    Ok(deserialized)
}
