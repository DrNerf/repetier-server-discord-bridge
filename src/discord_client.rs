use std::collections::HashMap;
use std::env;
use std::process;

const DISCORD_HOOK_VAR_NAME: &str = "DISCORD_BRIDGE_HOOK_URL";
const AVATAR_URL: &str = "https://avatars.githubusercontent.com/u/933919";
const USERNAME: &str = "Repetier Server";

pub async fn invoke_webhook(message: &String) -> String {
    let discord_hook_url = env::var(DISCORD_HOOK_VAR_NAME).unwrap_or_else(|_err| {
        println!(
            "{DISCORD_HOOK_VAR_NAME} environment variable not set! Shutting down Discord bridge."
        );
        process::exit(0x0100);
    });

    let client = reqwest::Client::new();
    let payload = create_discord_payload(message);
    let result = client.post(discord_hook_url).json(&payload).send().await;

    return if result.is_ok() {
        format!("Message proxied to discord: {message}")
    } else {
        format!(
            "Failed to proxy message to discord. Got an error respone: {}",
            result.unwrap_err().status().unwrap().as_str()
        )
    };
}

fn create_discord_payload(message: &String) -> HashMap<&str, String> {
    let mut payload = HashMap::new();
    payload.insert("avatar_url", String::from(AVATAR_URL));
    payload.insert("username", String::from(USERNAME));
    payload.insert("content", String::from(message));
    return payload;
}
