use rocket::tokio::task::spawn_blocking;
use rocket::tokio::time::{sleep, Duration};
use std::io;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/blocking_task")]
async fn blocking_task() -> io::Result<Vec<u8>> {
    // In a real app, use rocket::fs::NamedFile or tokio::fs::File.
    let vec = spawn_blocking(|| std::fs::read("data.txt"))
        .await
        .map_err(|e| io::Error::new(io::ErrorKind::Interrupted, e))??;

    Ok(vec)
}

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Waited for {} seconds", seconds)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/hello", routes![index])
        .mount("/", routes![blocking_task])
        .mount("/", routes![delay])
        .launch()
        .await?;

    Ok(())
}
