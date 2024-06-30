# Shuller

This is LIB for fetching links from Rule34 and other sites which you can implemented

## Example usage

```rust
async fn example() {
      use shuller::prelude::*;

      let instance: Posts = Params::init()
           .positive_tags(vec!["dark", ":)"])
           .negative_tags(vec!["ai_generated", "light"])
           .limit(3)
           .make_link()
           .search()
           .await
           .unwrap(); // or handle error
       println!("{:#?}", instance.get_s_urls())
    }
```

## Example usage with id

```rust
async fn example() {
      use shuller::prelude::*;

      let instance: Posts = Params::init()
           .id(10542274)
           .make_link()
           .search()
           .await
           .unwrap(); // or handle error
       println!("{:#?}", instance.get_s_urls())
    }
```

## Example with ext_data

```rust
    async fn check_mini_post() {
        let instance: Posts = Params::init()
            .id(10542274)
            .make_link()
            .search()
            .await
            .unwrap();
        println!("{:#?}", instance.get_url_ext())
    }
```

## Example with many ext_data

```rust

    #[tokio::test]
    async fn check_mini_post_many() {
        let instance: Posts = Params::init()
            .positive_tags(vec!["dark", "fish"])
            .negative_tags(vec!["ai_generated"])
            .limit(3)
            .make_link()
            .search()
            .await
            .unwrap();
        println!("{:#?}", instance.get_urls_ext())
    }
```
