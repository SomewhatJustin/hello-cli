use std::io;

#[tokio::main]
async fn main() {
    println!("Give me a string and I will reverse it oooooohhhh");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("fail!");

    let reversed_string = reverse_string(&user_input);
    println!("{reversed_string}");

    println!("Next: let's get some text from the internet");
    let internet_text = fetch_json("https://swapi.info/api/films").await.unwrap();
    println!("some text: {internet_text}")
}

fn reverse_string(string: &str) -> String {
    let mut reversed_string = String::new();
    for c in string.chars().rev() {
        reversed_string.push(c);
    }
    String::from(reversed_string)
}

async fn fetch_json(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;

    println!("text is {body}");

    Ok(body)
}
