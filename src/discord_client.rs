use std::collections::HashMap;

const DISCORD_HOOK: &str = "https://discord.com/api/webhooks/1070445082668445706/gkP1PuiPTlboQ7NCnobO3uVsWz1jNoY3EvYFaBzkuVRkclW0ShCpKLekWnBIYY9_snOz";
const AVATAR_URL: &str = "https://avatars.githubusercontent.com/u/933919";
const USERNAME: &str = "Repetier Server";

pub async fn invoke_webhook(message: &String) -> String {
    let client = reqwest::Client::new();
    let payload = create_discord_payload(message);
    let result = client.post(DISCORD_HOOK).json(&payload).send().await;

    return if result.is_ok() {
        format!("Message proxied to discord: {}", message)
    } else {
        format!(
            "Failed to proxy message to discord. Got an error respone: {}",
            result.unwrap_err().status().unwrap().as_str()
        )
    }
}

fn create_discord_payload(message: &String) -> HashMap<&str, String> {
    let mut payload = HashMap::new();
    payload.insert("avatar_url", String::from(AVATAR_URL));
    payload.insert("username", String::from(USERNAME));
    payload.insert("content", String::from(message));
    return payload
}