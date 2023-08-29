use reqwest::redirect::Policy;
use reqwest::StatusCode;
use serde::Deserialize;
use serde_json::Value;
/// Specify an argument with form IPv4 address, which is `Option<String>`
/// > 1. If the argument is None, acquire the information about the IP of the current location
/// >
/// > 2. If the argument is String, acquire the information about the specified IP address
/// 
/// # Examples
/// ```
/// # #[tokio::main]
/// # async fn main()->anyhow::Result<()> {
/// #  use serde_json::Value;
///    let r = geo_ip::geo_ip::<Value>(None).await?;
///    assert!(r.get("code").unwrap().as_str().unwrap() == "Success");
/// #   Ok(())
/// #  }
/// ```
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
