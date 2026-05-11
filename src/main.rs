use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::fs;
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

#[derive(Serialize, Deserialize)]
struct LogEntry {
    time: String,
    status: String,
    url: String,
    detail: String,
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
            save_to_json_log(&icon, &url, &detail);
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

fn save_to_json_log(status_icon: &str, url: &str, detail: &str) {
    let now = Local::now();
    let date_str = now.format("%Y-%m-%d").to_string();
    let file_name = format!("{}.json", date_str);
    
    let new_entry = LogEntry {
        time: now.format("%H:%M:%S").to_string(),
        status: status_icon.to_string(),
        url: url.to_string(),
        detail: detail.to_string(),
    };

    let mut log_list: Vec<LogEntry> = if let Ok(content) = fs::read_to_string(&file_name) {
        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    };

    log_list.push(new_entry);

    let json_data = serde_json::to_string_pretty(&log_list).expect("JSON 직렬화 실패");
    fs::write(file_name, json_data).expect("파일 쓰기 실패");
}