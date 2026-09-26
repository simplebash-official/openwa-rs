use openwa::types::CreateSessionRequest;
use openwa::OpenWAClient;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_url = env::var("OPENWA_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let api_key = env::var("OPENWA_API_KEY").expect("OPENWA_API_KEY must be set");

    let client = OpenWAClient::new(base_url, api_key)?;

    println!("Listing active sessions...");
    let sessions = client.sessions().list(None).await?;
    for sess in &sessions {
        println!(" - [{}] {} (Status: {:?})", sess.id, sess.name, sess.status);
    }

    let session_name = "rust-demo-session";
    println!("\nCreating session '{}'...", session_name);
    let new_sess = client
        .sessions()
        .create(CreateSessionRequest::new(session_name))
        .await?;

    println!("Session created: {} (ID: {})", new_sess.name, new_sess.id);

    // Fetch QR code
    println!("Fetching QR code for pairing...");
    match client.sessions().get_qr_code(&new_sess.id).await {
        Ok(qr) => println!("QR string (pass to terminal or UI): {}", qr.qr_code),
        Err(e) => println!("QR not ready yet or session already authenticated: {}", e),
    }

    Ok(())
}
