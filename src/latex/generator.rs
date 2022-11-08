use log::debug;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

const API_URL: &str = "https://rtex.probablyaweb.site/api/v2";

#[derive(Serialize, Clone, Debug)]
struct ApiCompilePayload<'a> {
    format: &'a str,
    code: &'a str,
    density: u32,
    quality: u32,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "status")]
enum CompileResponse {
    Success { log: String, filename: String },
    Error { description: String, log: String },
}

#[derive(Serialize, Clone)]
struct ApiFilePayload<'a> {
    filename: &'a str,
}

pub async fn generate_png_api(latex: &str) -> Result<(), ()> {
    let client = Client::new();

    let compile_response = client
        .post(API_URL)
        .json(&dbg!(ApiCompilePayload {
            format: "png",
            code: r"\documentclass{article}\begin{document}$a_n = { 1 \over 1 + \sqrt{n}}$\end{document}",
            density: 220,
            quality: 100,
        }))
        .send()
        .await
        .expect("failed to post compile request")
        .json::<CompileResponse>()
        .await
        .expect("failed to deserialize api response");

    match &compile_response {
        a @ CompileResponse::Success { filename, .. } => {
            debug!("{:?}", a);
            let file_bytes = client
                .get(format!("{API_URL}/{}", filename))
                .send()
                .await
                .expect("failed to get file")
                .bytes()
                .await
                .expect("failed to get bytes of image");

            let mut file = File::options()
                .create(true)
                .write(true)
                .open(format!("tex/message_id.png"))
                .unwrap();
            file.write_all(&file_bytes).unwrap();
        }
        a => debug!("{:?}", a),
    }

    Ok(())
}
