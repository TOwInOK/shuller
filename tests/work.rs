#[cfg(test)]
mod tests {
    use std::vec;

    use shuller::{prelude::*, rule34, rules::Rules}; // Импортируйте нужные модули

    #[tokio::test]
    async fn test_of_work() {
        let instance: Posts = Params::init()
            .positive_tags(vec!["dark"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .search()
            .await
            .unwrap();
        println!("{:#?}", instance.get_s_urls())
    }

    #[tokio::test]
    async fn check_mini_post() {
        let instance: Posts = Params::init().id(10542274).search().await.unwrap();
        println!("{:#?}", instance.get_url_ext())
    }

    #[tokio::test]
    async fn check_mini_post_many() {
        let instance: Posts = Params::init()
            .positive_tags(vec!["dark", "fish"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .search()
            .await
            .unwrap();
        println!("{:#?}", instance.get_urls_ext())
    }

    #[tokio::test]
    async fn test_macro() {
        let instance: Posts = rule34!(vec![], vec![]; 10542274).search().await.unwrap();
        println!("{:#?}", instance.get_s_url());
    }

    #[tokio::test]
    async fn test_via_enum() {
        let instance: Posts = Rules::Rule34(Params::init()).search().await.unwrap();
        println!("{:#?}", instance.get_s_url());
    }
}
