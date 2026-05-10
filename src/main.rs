use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use chrono::Local;
use health_check::core_check_site;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 5)]
    timeout: u64,
}

#[derive(Deserialize)]
struct Config {
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    
    let config_data = fs::read_to_string("config.json").expect("config 파일 X");
    let config: Config = serde_json::from_str(&config_data)?;
    
    let pb = ProgressBar::new(config.urls.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
        .progress_chars("#>-"));

    let mut handles = vec![];

    for url in config.urls {
        let pb_clone = pb.clone();
        let timeout = args.timeout;

        let handle = tokio::spawn(async move {
            let (icon, url, detail) = core_check_site(url, timeout).await;
            save_to_log(&icon, &url, &detail);
            pb_clone.inc(1);
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await?;
    }

    pb.finish_with_message("완료");
    Ok(())
}

fn save_to_log(status_icon: &str, url: &str, detail: &str) {
    let now = Local::now().format("%Y-%m-%d %H:%M:%S");
    let log_line = format!("[{}] {} {}: {}\n", now, status_icon, url, detail);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("check_log.txt")
        .unwrap();

    file.write_all(log_line.as_bytes()).unwrap();
}