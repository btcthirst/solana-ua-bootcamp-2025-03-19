use dotenv;
//use dotenvy;
use std::env;
fn main() {
    //let res = dotenvy::dotenv().is_ok();
    let res = dotenv::dotenv().ok();
    println!("Loaded .env file: {:?}", res);
    let secret_key = env::var("SECRET_KEY");
    match secret_key {
        Ok(key) => {
            println!("Secret key: {}", key);
        },
        Err(err) => {
            println!("{}",err);
        }
    }
}
