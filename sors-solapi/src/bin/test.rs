use sors_solapi::{client::Client, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::new("Samsung Smart Fridge <honbra@honbra.com>")?;

    // This is ~20 KB in prod T_T
    let token = client.sign_in("", "").await?.access_token;

    {
        let min_app_version = client.get_min_app_version().await?;
        println!("{min_app_version:#?}");
    }

    {
        let user_info = client.get_user_info(&token).await?;
        println!("{user_info:#?}");
    }

    Ok(())
}
