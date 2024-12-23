[ドキュメント](https://zenn.dev/uiuifree/books/rust-line-login-api)

## Install
```toml
[dependencies]
line-login-api = "0.1"
 
 ```

## Example

```rust
use std::env;
use dotenv::dotenv;
use line_login_api::LineLoginClient;

#[tokio::test]
async fn test_example() {
    let code = "";
    let redirect_uri = "";
    println!("test_example");
    assert!(true);
    let client = LineLoginClient::new(get_env("LINE_CLIENT_ID"), get_env("LINE_CLIENT_SECRET"));
    let Ok(res) = client.create_token(code, redirect_uri).await else {
        assert!(false, "create_token error");
        return;
    };
    println!("{:?}", res);
    let access_token = res.access_token;
    let refresh_token = res.refresh_token;
    let id_token = res.id_token;
    let Ok(res) = client.token_verify(&access_token).await else {
        return assert!(false, "token_verify error");
    };
    println!("{:?}", res);
    let Ok(res) = client.update_access_token(&refresh_token).await else {
        return assert!(false, "update_access_token error");
    };
    println!("{:?}", res);
    let access_token = res.access_token;
    let Ok(res) = client.id_token_verify(id_token.as_str(), None, None).await else {
        return assert!(false, "update_access_token error");
    };
    println!("{:?}", res);
    let Ok(res) = client.user_info(&access_token).await else {
        return assert!(false, "user_info error");
    };
    println!("{:?}", res);
    let Ok(res) = client.profile(&access_token).await else {
        return assert!(false, "profile error");
    };
    println!("{:?}", res);
    let Ok(res) = client.friend_ship(&access_token).await else {
        return assert!(false, "friend_ship error");
    };
    println!("{:?}", res);
}


fn get_env(key: &str) -> String {
    dotenv().ok();
    env::var(key).unwrap_or_else(|_| "".to_string())
}

```