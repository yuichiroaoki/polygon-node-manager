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
        let url = format!("https://google.com");
        let res = request::send_get_request(&url).await;
        assert_eq!(res.is_ok(), true);
    }
}
