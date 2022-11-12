pub(crate) mod web_api;
pub(crate) const TEMPLATE: &str = include_str!("template.tex");

use anyhow::Result;
use log::{info, warn};
use serenity::model::prelude::*;
use std::path::Path;
use std::time::Duration;
use subprocess::{Popen, PopenConfig, Redirection};

const API_URL: &str = "https://rtex.probablyaweb.site/api/v2";

const DENSITY: u32 = 400;
const QUALITY: u32 = 200;

pub async fn generate_png(code: &str, message_id: MessageId) -> Result<Vec<u8>> {
    std::fs::write(Path::new(&format!("out/{}.tex", message_id)), code)?;

    let (out, err) = run_command(&[
        "pdflatex",
        "-output-directory=out/",
        &format!("out/{}.tex", message_id),
    ])
    .await?;

    if let Some(out) = out {
        info!("compile output: {out}")
    }
    if let Some(err) = err {
        warn!("compile error: {err}")
    }

    let (out, err) = run_command(&[
        "magick",
        "-density",
        &format!("{DENSITY}"),
        "-quality",
        &format!("{QUALITY}"),
        &format!("out/{}.pdf", message_id),
        &format!("out/{}.png", message_id),
    ])
    .await?;

    Ok(std::fs::read(format!("out/{}.png", message_id))?)
}

async fn run_command(args: &[&str]) -> Result<(Option<String>, Option<String>)> {
    let mut proc = Popen::create(
        args,
        PopenConfig {
            stdin: Redirection::Pipe,
            stdout: Redirection::Pipe,
            stderr: Redirection::Pipe,
            ..Default::default()
        },
    )?;

    // proc.wait_timeout(Duration::from_secs(1))?;
    // TODO: find out why latex compiler is blocking
    Ok(proc.communicate(Some("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n"))?)
}
