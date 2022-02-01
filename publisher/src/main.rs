mod message;
mod redis_publisher;

extern crate redis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("service started");

    redis_publisher::publish_message(message::Message::new(
        String::from("Ping"))
    )?;

    println!("published");
    Ok(())
}
