use super::post::Posts;

#[derive(Debug, PartialEq)]
pub struct Link {
    url: String,
}

impl Link {
    pub fn init(url: String) -> Self {
        Self { url }
    }

    fn url(&self) -> &str {
        &self.url
    }

    pub async fn search(&self) -> Result<Posts, reqwest::Error> {
        let posts: Posts = reqwest::get(self.url()).await?.json().await?;
        Ok(posts)
    }
}
