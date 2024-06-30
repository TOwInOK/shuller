use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Posts(Vec<Post>);

/// Post - 1 picture
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    /// picture in miniature
    pub preview_url: String,
    /// picture resized by default
    /// W - `sample_wight`
    /// H - `sample_hight`
    pub sample_url: String,
    /// raw picture
    /// W - `wight`
    /// H - `hight`
    pub file_url: String,
    /// Hash of picture
    pub hash: String,
    /// W
    pub width: i64,
    /// H
    pub height: i64,
    /// Unique ID
    pub id: i64,
    /// File name
    pub image: String,
    /// Author
    pub owner: String,
    /// Linked ID
    pub parent_id: i64,
    /// has resized ?
    pub sample: bool,
    /// H
    pub sample_height: i64,
    /// W
    pub sample_width: i64,
    /// Home many people come on this image?
    pub score: i64,
    /// Tags of picture
    pub tags: String,
    /// Used for translation (in my case)
    pub has_notes: bool,
    // pub comment_count: i64,
}

impl Posts {
    /// get `preview_url` of all posts
    pub fn get_p_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.preview_url));
        urls
    }
    /// get `sample_url` of all posts
    pub fn get_s_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.sample_url));
        urls
    }
    /// get `file_url` of all posts
    pub fn get_f_urls(&self) -> Vec<&str> {
        let mut urls: Vec<&str> = vec![];
        self.0.iter().for_each(|x| urls.push(&x.file_url));
        urls
    }

    /// get first `preview_url`
    pub fn get_first_p_urls(&self) -> Option<&str> {
        self.0.first().and_then(|x| Some(x.preview_url.as_str()))
    }
    /// get first `sample_url`
    pub fn get_first_s_urls(&self) -> Option<&str> {
        self.0.first().and_then(|x| Some(x.sample_url.as_str()))
    }
    /// get first `file_url`
    pub fn get_first_f_urls(&self) -> Option<&str> {
        self.0.first().and_then(|x| Some(x.file_url.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        link::link::MakeLink,
        params::{params::Params, rule::Rule34},
    };

    #[tokio::test]
    async fn dwl() {
        let binding = Params::init().make_link().search().await.unwrap();
        let result = binding.get_f_urls();
        println!("{:#?}", result)
    }
}
