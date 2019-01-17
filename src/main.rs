extern crate maxminddb;

mod emoji;

use std::net::IpAddr;
use std::str::FromStr;
use maxminddb::geoip2;

static DB: &'static str = "/usr/local/share/GeoIP/GeoLite2-City.mmdb";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];
    let ip: IpAddr = FromStr::from_str(input).unwrap();

    let r = maxminddb::Reader::open_readfile(DB);
    if let Err(err) = r {
        panic!(format!("error opening mmdb: {:?}", err));
    }
    let ip: geoip2::City = r.unwrap().lookup(ip).unwrap();

    let country = ip.country.unwrap().names.unwrap();
    let country_name = country.get("en").unwrap();
    let mut subdivs = ip.subdivisions.unwrap();
    let subdiv = subdivs.pop().unwrap().names.unwrap();
    let subdiv_name = subdiv.get("en").unwrap();
    let city = ip.city.unwrap().names.unwrap();
    let city_name = city.get("en").unwrap();
    println!("{}  {} -- {}, {}", emoji::flag(country_name), country_name, city_name, subdiv_name);
}

