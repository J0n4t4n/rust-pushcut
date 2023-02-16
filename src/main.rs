use std::collections::HashMap;
use std::env;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)] // to properly serialize custom without "custom" tag
#[allow(dead_code)]
enum Sound {
    VibrateOnly,
    System,
    Subtle,
    Question,
    JobDone,
    Problem,
    Loud,
    Lasers,
    Custom(String),
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutUrlBackgroundOptions {
    http_method: String,
    http_content_type: Option<String>,
    http_header: Option<Vec<HashMap<String, String>>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutAction {
    name: Option<String>,
    input: Option<String>,
    keep_notification: bool,
    shortcut: Option<String>,
    homekit: Option<String>,
    run_on_server: bool,
    online: Option<String>,
    url: Option<String>,
    url_background_options: Option<PushcutUrlBackgroundOptions>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutRequest {
    text: Option<String>,
    title: Option<String>,
    default_action: Option<PushcutAction>,
    actions: Option<Vec<PushcutAction>>,
    sound: Option<Sound>,
    image: Option<String>,      // Name or web URL
    image_data: Option<String>, // base64 encoded image
    input: Option<String>,
    devices: Option<Vec<String>>,
    is_time_sensitive: Option<bool>,
    thread_id: Option<String>,
    delay: Option<String>,            // extended feature, can't test
    schedule_timestamp: Option<u128>, // extended feature, can't test
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PushcutResponse {
    id: String,
    message: String,
    notification_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = PushcutRequest {
        text: Some(String::from("NACHRICHT")),
        title: Some(String::from("Dieser Titel")),
        default_action: None,
        actions: None,
        sound: Some(Sound::Custom(String::from("test"))),
        image: None,
        image_data: None,
        input: None,
        devices: None,
        is_time_sensitive: None,
        thread_id: None,
        delay: None,
        schedule_timestamp: None,
    };

    let secret = env::var_os("PUSHCUT_SECRET")
        .expect("$PUSHCUT_SECRET is required")
        .into_string()
        .unwrap();

    let js = serde_json::to_string_pretty(&body).unwrap();
    println!("{js:#?}");

    let resp = client
        .post(format!(
            "https://api.pushcut.io/{secret}/notifications/test"
        ))
        .json(&body)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}
