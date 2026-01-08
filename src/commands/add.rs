use std::{collections::HashMap, time::Duration};

use indicatif::ProgressBar;
use inquire::Select;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{
    github::{Release, ReleaseAsset, client::Client},
    store::{
        Database,
        models::{Config, NewRepository},
    },
};

pub async fn download_package<'a>(
    args: (String, String, Option<String>, Option<String>),
    config: &Config,
    package: &ReleaseAsset,
) -> anyhow::Result<NewRepository<'a>> {
    let (repo, tag, name, directory) = args;
    let parts: Vec<&str> = repo.split('/').collect();
    let new_repo = NewRepository {
        owner: parts.first().unwrap(),
        name: parts.iter().next().unwrap(),
        package: &package.name,
        location: &format!(
            "{}/{}",
            directory.unwrap_or(config.installation_directory.to_string_lossy().to_string()),
            name.unwrap_or(package.name.clone())
        ),
        tag: &repo,
        locked: false,
    };

    let file = File::open(new_repo.location).await?;

    file.write();

    todo!()
}

pub async fn run(
    args: (String, Option<String>, Option<String>, Option<String>),
    client: &Client,
    db: &Database,
) -> anyhow::Result<()> {
    let (repo, tag, name, directory) = args;
    let config = db.get_config().await?;
    let release = client.get_release(&repo, tag).await?;
    let options: HashMap<String, ReleaseAsset> = release
        .assets
        .into_iter()
        .map(|o| return (o.name.clone(), o))
        .collect();

    let package = Select::new("Pick available version", options.keys().collect()).prompt()?;

    Ok(())
}
