# Shuller

Library for link fetching from Rule34 and other implementable sites

## Example usage

```rust
async fn example() {
    use shuller::prelude::*;

    let instance: Posts = Params::init()
        .positive_tags(vec!["dark", ":)"])
        .negative_tags(vec!["ai_generated", "light"])
        .limit(3)
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
        .search()
        .await
        .unwrap(); // or handle error
    println!("{:#?}", instance.get_s_urls())
}
```

## Example with ext_data

```rust
async fn check_mini_post() {
    use shuller::prelude::*;

    let instance: Posts = Params::init()
        .id(10542274)
        .search()
        .await
        .unwrap();
    println!("{:#?}", instance.get_url_ext())
}
```

## Example with many ext_data

```rust
async fn check_mini_post_many() {
    use shuller::prelude::*;

    let instance: Posts = Params::init()
        .positive_tags(vec!["dark", "fish"])
        .negative_tags(vec!["ai_generated"])
        .limit(3)
        .search()
        .await
        .unwrap();
    println!("{:#?}", instance.get_urls_ext())
}
```

## Example with macros

```rust
async fn test_macro() {
    use shuller::prelude::*;

    // Positive , Negative, Limit, Start Page
    let instance: Posts = rule34!(vec!["dark", "fish"], vec!["ai_generated"], 3, 1).search().await.unwrap();

    // Positive , Negative, ID
    let instance: Posts = rule34!(vec![], vec![]; 10542274).search().await.unwrap();

    // ID
    let instance: Posts = rule34!(; 10542274).search().await.unwrap();
    println!("{:#?}", instance.get_s_url());
}
```

## Example with enum builder

```rust
async fn test_via_enum() {
    use shuller::prelude::*;
    
    let instance: Posts = Rules::Rule34(Params::init()).search().await.unwrap();
    println!("{:#?}", instance.get_s_url());
}
```
