use reqwest::redirect::Policy;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
pub async fn geo_ip<T: Deserialize<'static>>(ipv4: Option<String>) -> anyhow::Result<T> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .redirect(Policy::none())
        .build()?;
    let api = if let Some(addr) = ipv4 {
        format!("https://qifu-api.baidubce.com/ip/geo/v1/district?ip={addr}")
    } else {
        "https://qifu-api.baidubce.com/ip/local/geo/v1/district".to_owned()
    };
    let res = client
        .get(api)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:109.0) Gecko/20100101 Firefox/116.0",
        )
        .send()
        .await?;
    if res.status() == StatusCode::OK {
        let body = res.json::<Value>().await?;
        return Ok(T::deserialize(body)?);
    } else {
        return Err(anyhow::anyhow!("http response is not ok"));
    }
}
