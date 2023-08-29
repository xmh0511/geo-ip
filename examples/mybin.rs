use geo_ip;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
struct Info {
    accuracy: String,
    adcode: String,
    areacode: String,
    asnumber: String,
    city: String,
    continent: String,
    country: String,
    district: String,
    isp: String,
    lat: String,
    lng: String,
    owner: String,
    prov: String,
    radius: String,
    source: String,
    timezone: String,
    zipcode: String,
}
#[derive(Deserialize, Debug)]
struct Data {
    charge: bool,
    code: String,
    coordsys: String,
    data: Info,
    ip: String,
    msg: String,
}
#[tokio::main]
async fn main() {
    let r = geo_ip::geo_ip::<Data>(None).await.unwrap();
	println!("{r:#?}");
}
