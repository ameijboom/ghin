use serde::Deserialize;

pub mod client;

#[derive(Deserialize, Debug)]
pub struct Release {
    #[serde(rename = "name")]
    pub version: String,
    pub url: String,
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Deserialize, Debug)]
pub struct ReleaseAsset {
    pub name: String,
    #[serde(rename = "browser_download_url")]
    pub url: String,
    pub digest: Option<String>,
}
