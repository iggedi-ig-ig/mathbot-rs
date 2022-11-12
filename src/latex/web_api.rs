use crate::latex::{API_URL, DENSITY, QUALITY};
use anyhow::{Error, Result};
use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serenity::model::prelude::*;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Clone, Debug)]
struct ApiCompilePayload<'a> {
    format: &'a str,
    code: &'a str,
    density: u32,
    quality: u32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase", tag = "status")]
#[allow(dead_code)]
enum CompileResponse {
    Success { log: String, filename: String },
    Error { description: String, log: String },
}

#[derive(Serialize, Clone)]
struct ApiFilePayload<'a> {
    filename: &'a str,
}

pub async fn generate_png_api(code: &str, message_id: MessageId) -> Result<Vec<u8>> {
    let client = Client::new();

    let compile_response = client
        .post(API_URL)
        .json(&dbg!(ApiCompilePayload {
            format: "png",
            code,
            density: DENSITY,
            quality: QUALITY,
        }))
        .send()
        .await?
        .json::<CompileResponse>()
        .await?;

    match &compile_response {
        a @ CompileResponse::Success { filename, .. } => {
            debug!("{:?}", a);
            let file_bytes = client
                .get(format!("{API_URL}/{}", filename))
                .send()
                .await?
                .bytes()
                .await?;

            let mut file = File::options()
                .create(true)
                .write(true)
                .open(format!("tex/{}.png", message_id))?;
            file.write_all(&file_bytes)?;

            Ok(file_bytes.to_vec())
        }
        a => Err(Error::msg(format!("{a:?}"))),
    }
}
