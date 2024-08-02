#[cfg(test)]
mod tests {
    use std::vec;

    use shuller::prelude::*; // Импортируйте нужные модули
    #[tokio::test]
    async fn test_of_work() {
        let instance: Posts = R34Params::init()
            .positive_tags(vec!["dark"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .download()
            .await
            .unwrap();
        assert!(instance.get_s_urls().len() == 3)
    }

    #[tokio::test]
    async fn check_mini_post() {
        let instance: Posts = R34Params::init().id(10542274).download().await.unwrap();
        assert!(instance.get_url_ext().is_some())
    }

    #[tokio::test]
    async fn check_mini_post_many() {
        let instance: Posts = R34Params::init()
            .positive_tags(vec!["dark", "fish"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .download()
            .await
            .unwrap();
        assert!(instance.get_urls_ext().len() == 3)
    }

    #[tokio::test]
    async fn test_macro_normal() {
        let instance = R34!(
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        )
        .download()
        .await
        .unwrap();
        assert!(instance.get_f_urls().len() == 2)
    }
    #[tokio::test]
    async fn test_macro_download() {
        let instance = R34!(D;
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        )
        .unwrap();
        assert!(instance.get_f_urls().len() == 2)
    }
    #[tokio::test]
    async fn test_macro_url() {
        let instance = R34!(U;
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        );
        assert!(instance.is_special())
    }
    #[cfg(feature = "rand")]
    #[tokio::test]
    async fn test_macro_random_url() {
        let instance = R34!(R; U);
        assert!(instance.is_special())
    }
    #[cfg(feature = "rand")]
    #[tokio::test]
    async fn test_macro_random_params() {
        let instance = R34!(R;).url_generate();
        assert!(instance.is_special())
    }
    #[cfg(feature = "rand")]
    #[tokio::test]
    async fn test_macro_random_download() {
        let instance = R34!(R; D).unwrap();
        let url = uller::Url::parse(instance.get_f_url().unwrap()).unwrap();
        assert!(url.is_special())
    }
}
