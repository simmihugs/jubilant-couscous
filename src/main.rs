use reqwest::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://sw-api.starnavi.io/species/";
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response: {}", body);
    } else {
        println!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
