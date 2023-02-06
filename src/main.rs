use std::collections::HashMap;
use tokio::task::block_in_place;
use warp::Filter;
use futures::executor::block_on;

mod discord_client;

const PORT: u16 = 3030;

#[tokio::main]
async fn main() {
    // GET /push/{message}
    let push = warp::path("push")
        .and(warp::query::<HashMap<String, String>>())
        .map(|map: HashMap<String, String>| handle_push_notification(map.get("message").unwrap()));

    println!("Discord bridge server running on: {PORT}");

    warp::serve(push).run(([127, 0, 0, 1], PORT)).await;
}

fn handle_push_notification(message: &String) -> String {
    
    let result = block_in_place(move || {
        return block_on(discord_client::invoke_webhook(message))
    });
    println!("{}", result);
    return result;
}
