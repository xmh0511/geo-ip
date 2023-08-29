````rust
use geo_ip;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let r = geo_ip::geo_ip::<Value>(None).await.unwrap();
	println!("{r:#?}");

	let r = geo_ip::geo_ip::<Value>(Some("1.1.1.1".to_owned())).await.unwrap();
}
````