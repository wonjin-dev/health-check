use reqwest;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let urls = vec![
        "https://www.google.com",
        "https://www.github.com",
        "https://www.naver.com",
    ];

    let mut handles = vec![];

    for url in urls {
        let handle = tokio::spawn(async move {
            match reqwest::get(url).await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("✅ {}", url);
                    } else {
                        println!("⚠️ {}", url);
                    }
                }
                Err(_) => println!("❌ {}", url),
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    Ok(())
}