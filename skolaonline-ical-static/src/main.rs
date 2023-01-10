use clap::Parser;
use tokio::io::AsyncWriteExt;

#[derive(Debug, clap::Parser)]
struct Cli {
    username: String,
    password: String,
    // date_from: chrono::NaiveDate,
    // date_to: chrono::NaiveDate,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    let today = chrono::Local::now().date_naive();
    let date_from = today - chrono::Duration::days(7);
    let date_to = today + chrono::Duration::days(30);

    let calendar =
        skolaonline_ical::fetch_calendar(&args.username, &args.password, date_from, Some(date_to))
            .await?;

    let mut file = tokio::fs::File::create("./calendar.ics").await?;
    file.write_all(calendar.to_string().as_bytes()).await?;

    Ok(())
}
