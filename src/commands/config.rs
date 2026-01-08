use colored::Colorize;
use std::path::PathBuf;

use crate::store::{Database, models::NewConfig};

pub async fn run(args: (Option<PathBuf>, Option<PathBuf>), db: &Database) -> anyhow::Result<()> {
    let (install, temp) = (
        args.0.map(|path| path.to_string_lossy().to_string()),
        args.1.map(|path| path.to_string_lossy().to_string()),
    );

    if install.is_none() && temp.is_none() {
        let c = db.get_config().await?;

        println!(
            "Current installation directory: {}",
            c.installation_directory.to_string_lossy().bold()
        );

        println!(
            "Current temp directory: {}",
            c.temp_directory.to_string_lossy().bold()
        );

        return Ok(());
    }

    let old = db
        .set_config(NewConfig {
            installation_directory: install.clone(),
            temp_directory: temp.clone(),
        })
        .await?;

    if let Some(old) = old {
        if let Some(new) = &install {
            println!(
                "{} -> {}",
                old.installation_directory.to_string_lossy().red(),
                new.to_string().green()
            )
        }

        if let Some(new) = &temp {
            println!(
                "{} -> {}",
                old.temp_directory.to_string_lossy().red(),
                new.to_string().green()
            )
        }

        return Ok(());
    }

    println!("No changes detected.");

    Ok(())
}
