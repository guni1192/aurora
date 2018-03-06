extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_derive;

use self::serde_json::{Value, Error};

pub struct PackageInfo {
    ID: i32,
    Name: String,
    FirstSubmited: i32,
    URL: String,
    Maintainer: String,
    Version: String,
    NumVotes: i32,
    PackageBaseID: i32,
    URLPath: String,
    LastModified: i32,
    OutOfDate: String,
}

#[derive(Serialize, Deserialize)]
pub struct RpcResponse {
    Type: String,
    resultcount: i32,
    version: i32,
    result: Vec<PackageInfo>,
}

pub fn get_query(keyword: String) -> RpcResponse {
    let base_url = "https://aur.archlinux.org/rpc.php";
    let search_url = format!("{}/?v=5&type=search&arg={}", base_url, keyword);
    let mut responce = self::reqwest::get(&search_url).unwrap();

    let mut body = String::new();
    let query: RpcResponse = serde_json::from_str(&mut body).unwrap();
    // let mut query: Vec<PackageInfo> = v.get("result");
    // match v.get("results") {
    //     Some(pkgs) => {
    //         query = pkgs;
    //     },
    //     _ => panic!("hoge"),
    // };
    query
}
