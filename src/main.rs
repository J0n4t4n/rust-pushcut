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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
enum PushcutActionType {
    Shortcut(String), // name of shortcut to run
    Homekit(String),  // name of homekit scene to execute
    Url(String),      // URL that this action should open
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(dead_code)]
enum PushcutHttpMethod {
    Get,
    Post,
    Put,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
struct PushcutHttpHeader {
    key: String,
    value: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutUrlBackgroundOptions {
    http_method: Option<PushcutHttpMethod>,
    http_content_type: Option<String>,
    http_header: Option<Vec<PushcutHttpHeader>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PushcutAction {
    name: Option<String>,            // Name of the action
    input: Option<String>,           // Input value, passed to action
    keep_notification: Option<bool>, // true will not dismiss notification after action
    run_on_server: Option<bool>,     // true will run shortcut/homekit action on automation server
    online: Option<String>, // name of online automation to execute ("Integration: Trigger")
    url_background_options: Option<PushcutUrlBackgroundOptions>, // configuration for background web request
    #[serde(flatten)]
    action: PushcutActionType,
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
        text: None,
        title: None,
        default_action: None,
        actions: Some(vec![PushcutAction {
            name: Some(String::from("Test Action")),
            input: None,
            keep_notification: Some(true),
            run_on_server: None,
            online: None,
            url_background_options: Some(PushcutUrlBackgroundOptions {
                http_method: None,
                http_content_type: None,
                http_header: None,
            }),
            action: PushcutActionType::Url(String::from("https://google.de")),
        }]),
        sound: Some(Sound::Custom(String::from("test"))),
        image: None,
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
