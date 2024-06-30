#[cfg(test)]
mod tests {
    use shuller::prelude::*; // Импортируйте нужные модули

    #[tokio::test]
    async fn test_of_work() {
        let instance: Posts = Params::init()
            .positive_tags(vec!["dark"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .make_link()
            .search()
            .await
            .unwrap();
        println!("{:#?}", instance.get_s_urls())
    }
}
