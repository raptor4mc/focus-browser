#[tokio::main]
async fn main() {
    println!("P2 fetch: requesting https://example.com...");
    let resp = reqwest::get("https://example.com")
        .await
        .expect("fetch failed");
    println!("Status: {}", resp.status());
    let text = resp.text().await.expect("read body failed");
    println!("Length: {} bytes", text.len());
    println!("First 200 chars: {}", &text[..text.len().min(200)]);
}
