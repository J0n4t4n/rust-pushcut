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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)] // to properly serialize custom without "custom" tag
#[allow(dead_code)]
enum PushcutImage {
    Image(String),     // Name or web URL
    ImageData(String), // base64 encoded image
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutUrlBackgroundOptions {
    http_method: Option<String>,
    http_content_type: Option<String>,
    http_header: Option<Vec<HashMap<String, String>>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutAction {
    name: Option<String>,            // Name of the action
    input: Option<String>,           // Input value, passed to action
    keep_notification: Option<bool>, // true will not dismiss notification after action
    shortcut: Option<String>,        // name of shortcut to run
    homekit: Option<String>,         // name of homekit scene to execute
    run_on_server: Option<bool>,     // true will run shortcut/homekit action on automation server
    online: Option<String>, // name of online automation to execute ("Integration: Trigger")
    url: Option<String>,    // URL that this action should open
    url_background_options: Option<PushcutUrlBackgroundOptions>, // configuration for background web request
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutRequest {
    id: Option<String>,
    text: Option<String>,
    title: Option<String>,
    default_action: Option<PushcutAction>,
    actions: Option<Vec<PushcutAction>>,
    sound: Option<Sound>,
    image: Option<PushcutImage>,
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

async fn send_notification(name: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let body = PushcutRequest {
        id: Some(String::from("Test")),
        text: Some(String::from("Nachricht")),
        title: Some(String::from("Dieser Titel")),
        default_action: None,
        actions: Some(vec![PushcutAction {
            name: Some(String::from("Test Action")),
            input: None,
            keep_notification: Some(true),
            shortcut: None,
            homekit: None,
            run_on_server: None,
            online: None,
            url: Some(String::from("https://google.de")),
            url_background_options: Some(PushcutUrlBackgroundOptions {
                http_method: Some(String::from("GET")),
                http_content_type: Some(String::from("application/json")),
                http_header: None,
            }),
        }]),
        sound: Some(Sound::Custom(String::from("test"))),
        image: Some(PushcutImage::Image(String::from("https://heise.cloudimg.io/v7/_www-heise-de_/imgs/18/3/7/0/2/8/8/2/shutterstock_376943356-9a1dc8b9e51de9a0.jpeg?force_format=avif%2Cwebp%2Cjpeg&org_if_sml=1&q=50&width=516"))),
        input: None,
        devices: None,
        is_time_sensitive: None,
        thread_id: None,
        delay: None,
        schedule_timestamp: None,
    };

    let js = serde_json::to_string_pretty(&body).unwrap();
    println!("{js:#?}");

    let resp = client
        .post(format!("https://api.pushcut.io/v1/notifications/{name}"))
        .header("API-Key", api_key)
        .json(&body)
        .send()
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}

async fn get_devices(api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client
        .get(format!("https://api.pushcut.io/v1/devices"))
        .header("API-Key", api_key)
        .send()
        .await?
        .json::<Vec<HashMap<String, String>>>()
        .await?;
    println!("{resp:#?}");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = env::var_os("PUSHCUT_API_KEY")
        .expect("$PUSHCUT_API_KEY is required")
        .into_string()
        .unwrap();

    send_notification("test", &api_key).await
    //get_devices(&api_key).await
}
