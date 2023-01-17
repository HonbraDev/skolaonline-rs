use anyhow::Result;
use clap::Parser;
use comfy_table::Table;
use skolaonline::client::SOClient;
use std::collections::HashMap;

#[derive(Debug, clap::Parser)]
struct Cli {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();

    let client = SOClient::new(&args.username, &args.password);

    let grades = client.get_grades().await?.hodnoceni;
    let grade_types = client.get_grade_types().await?;
    let subjects = client.get_subjects().await?;

    let grades_by_subject: HashMap<String, Vec<(f64, f64)>> = grades
        .into_iter()
        .filter_map(|grade| {
            if let Some(percent) = grade.procenta {
                let subject = subjects.get(&grade.predmet_id)?;
                let grade_type = grade_types.get(&grade.druh_hodnoceni_id)?;
                Some((subject.nazev.clone(), (percent, grade_type.vaha)))
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut map, (subject, grade)| {
            map.entry(subject).or_insert_with(Vec::new).push(grade);
            map
        });

    let weighted_averages: HashMap<String, f64> = grades_by_subject
        .into_iter()
        .map(|(subject, grades)| {
            let (sum, weight_sum): (f64, f64) = grades
                .into_iter()
                .fold((0.0, 0.0), |(sum, weight_sum), (grade, weight)| {
                    (sum + grade * weight, weight_sum + weight)
                });

            (subject, sum / weight_sum)
        })
        .collect();

    let mut weighted_averages: Vec<(String, f64)> = weighted_averages.into_iter().collect();
    weighted_averages.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    let mut table = Table::new();
    table.set_header(vec!["Subject", "Average grade"]);

    for (subject, grade) in weighted_averages {
        table.add_row(vec![subject, format!("{:.2}%", grade)]);
    }

    println!("{table}");

    Ok(())
}
