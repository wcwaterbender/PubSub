mod message;
mod message_handler;
mod redis_subscriber;

extern crate redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    if let Err(error) = redis_subscriber::subscribe(String::from("stats")) {
        println!("{:?}", error);
        panic!("{:?}", error);
    } else {
        println!("connected to queue");
    }

    Ok(())
}
