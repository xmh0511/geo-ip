use std::str::FromStr;

use geo_ip;
use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Data{
	address:String,
	code:u16,
	ip:String,
	#[serde(rename = "isDomain")]
	is_domain:u8,
	rs:u8
}
#[tokio::main]
async fn main(){
	let r = geo_ip::geo_ip::<Value>(None).await.unwrap();
	println!("{r:#?}");
}