extern crate reqwest;
// use std::io;

extern crate serde_json;
use serde_json::{Value, Error};

mod math;
use math::AddMean;

extern crate mammut;
extern crate toml;

use std::io;
use std::fs::File;
use std::io::prelude::*;

use mammut::{Data, Mastodon, Registration};
use mammut::apps::{AppBuilder, Scopes};
use mammut::status_builder::StatusBuilder;
use mammut::status_builder::Visibility;

fn main() -> Result<(),String> {

    let diff = run()?;

    // register();

    let mastodon = match File::open("mastodon-data.toml") {
        Ok(mut file) => {
            let mut config = String::new();
            file.read_to_string(&mut config).unwrap();
            let data: Data = toml::from_str(&config).unwrap();
            Mastodon::from_data(data)
        },
        Err(_) => register(),
    };

    // let you = mastodon.verify_credentials().unwrap();
    // println!("{:#?}", you);
    

    if (diff.abs() > 1000i64 ) {
        let mut status = StatusBuilder::new(format!("BTC/JPYが {} 変動しました", diff).into());
        status.visibility = Some(Visibility::Unlisted);
        mastodon.new_status(status).map_err(|err|  format!("post status failed: {}", err.to_string()) )?;
    }
    Ok(())
}

fn run() -> Result<i64, String>  {
    let url = "https://api.cryptowat.ch/markets/bitflyer/btcjpy/ohlc?periods=60";
    let mut res = reqwest::get(url).map_err(|err|  format!("Download failed: {}", err.to_string()) )?;
    let body = res.text().map_err(|err|  format!("get text failed: {}", err.to_string()) )?;
    let v: Value = serde_json::from_str(&body).map_err(|err|  format!("parse json failed: {}", err.to_string()) )?;
    let vv = &v["result"]["60"];

    let arr_list: Vec<i64> = vv.as_array().ok_or(format!("parse json array failed"))?.iter().map ( |v| v[3].as_i64().expect("parase i64") ).collect();
    // println!("{:?}", arr_list[0]);
    // println!("{:?}", &arr_list[1..100].to_vec().mean());
    let len = arr_list.len();
    let latest = arr_list[len-1];
    let latest_mean = &arr_list[len-30..len-2].to_vec().mean();
    let diff = latest - latest_mean;
    println!("{}", latest);
    println!("{}", latest_mean);
    println!("{}", diff);

    Ok(diff)
}

fn register() -> Mastodon {
    let app = AppBuilder {
        client_name: "cryptdon-bots",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::All,
        website: Some("https://github.com/nacika-ins"),
    };

    let mut registration = Registration::new("https://crypto-don.net");
    registration.register(app).unwrap();;
    let url = registration.authorise().unwrap();

    println!("Click this link to authorize on Mastodon: {}", url);
    println!("Paste the returned authorization code: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string()).unwrap();

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon).unwrap();
    let mut file = File::create("mastodon-data.toml").unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    mastodon
}
