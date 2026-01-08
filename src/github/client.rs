use crate::github::Release;

pub struct Client {
    api: reqwest::Client,
}

impl Client {
    const BASE_URL: &str = "https://api.github.com";
    const USER_AGENT: (&str, &str) = ("User-Agent", "ameijboom/ghin");

    pub fn new() -> Self {
        Self {
            api: reqwest::Client::new(),
        }
    }

    pub async fn download_release(&self, url: &str) -> anyhow::Result<[u8]> {
        let
        todo!()
    }
}

    pub async fn get_release(&self, repo: &str, tag: Option<String>) -> anyhow::Result<Release> {
        let url = match tag {
            Some(tag) => format!("{}/repos/{}/releases/{}", Self::BASE_URL, repo, tag),
            None => format!("{}/repos/{}/releases/latest", Self::BASE_URL, repo),
        };

        Ok(self
            .api
            .get(&url)
            .header(Self::USER_AGENT.0, Self::USER_AGENT.1)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn list_releases(&self, repo: &str) -> anyhow::Result<Vec<Release>> {
        Ok(self
            .api
            .get(&format!("{}/repos/{}/releases", Self::BASE_URL, repo))
            .header(Self::USER_AGENT.0, Self::USER_AGENT.1)
            .send()
            .await?
            .json()
            .await?)
    }
}
