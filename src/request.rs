pub async fn send_get_request(url: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let body = res.text().await?;
    Ok(body)
}

#[cfg(test)]
mod test {
    use crate::request;

    #[tokio::test]
    async fn test_send_get_request() {
        let url = "https://api.1inch.io/v4.0/1/healthcheck";
        let res = request::send_get_request(&url).await;
        assert_eq!(res.unwrap(), "{\"status\":\"OK\"}");
    }
}
