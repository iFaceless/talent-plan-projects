use serde::{Serialize, Deserialize};
use serde_json;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    println!("Hello, world!");
    let user = User {
        id: 100,
        token: "abc".to_owned(),
        email: "abc@gmail.com".to_owned(),
        nickname: "0xE8551CCB".to_owned(),
    };

    let s = serde_json::to_string(&user)?;
    // {"id":100,"token":"abc","email":"abc@gmail.com","nickname":"0xE8551CCB"}
    println!("{}", &s);

    let u: User = serde_json::from_str(&s)?;
    println!("{:?}", u);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u64,
    token: String,
    email: String,
    nickname: String,
}
