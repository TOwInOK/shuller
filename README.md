# Shuller

Library for link fetching from Rule34 and other implementable sites

**Note** for learn more info about this lib check [doc.rs on crates.io](https://crates.io/crates/shuller)

# Example __**(use macro instead)**__
```rust
use shuller::prelude::*;
async fn example() {
    let instance: Posts = R34Params::init()
        .positive_tags(vec!["dark", "fish"])
        .negative_tags(vec!["ai_generated"])
        .limit(3)
        .download() // or url_generate()
        .await
        .unwrap();
        println!("{:#?}", instance.get_urls_ext())
}
```
# Example via macro
  * Generate Url by params and manual fetch data structure Posts
    ```rust
    use shuller::prelude::*;
    async fn macro_normal() {
        let instance = R34!(
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        )
        .download()
        .await
        .unwrap();
    }
    ```
  * Fetch data structure Posts, macro include
    ```rust
    use shuller::prelude::*;
    async fn macro_download() {
        let instance = R34!(D;
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        )
        .unwrap();
    }
    ```
  * Generate Url via macro, it's just url.
    ```rust
    use shuller::prelude::*;
    async fn macro_url() {
        let instance = R34!(U;
            p = vec!["dark", "fish"],
            n = vec!["ai_generated"],
            limit = 2,
            page = 2
        );
    }
    ```
