use sors_solapi::{client::Client, error::Error};

// This is ~20 KB in prod T_T
const TOKEN: &str = "";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new("Samsung Smart Fridge <honbra@honbra.com>")?;

    {
        let min_app_version = client.get_min_app_version().await?;
        println!("{min_app_version:#?}");
    }

    {
        let user_info = client.get_user_info(TOKEN).await?;
        println!("{user_info:#?}");
    }

    Ok(())
}
