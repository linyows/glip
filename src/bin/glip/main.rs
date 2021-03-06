// glip - Copyright 2019 Tomohisa Oda

#[macro_use]
extern crate clap;
extern crate glip;

use clap::{App, Arg, AppSettings};
use glip::GeoIp;
use std::process;

fn run(ip: &str, format: &str) -> i32 {
    let geoip = GeoIp::new(ip);
    if let Err(err) = geoip {
        println!("Error: {}", err.to_string());
        return 1;
    }

    let g = geoip.unwrap();
    match format {
        "json" => println!("{{\"flag\":\"{}\",\"contry\":\"{}\",\"city\":\"{}\",\"subdivision\":\"{}\"}}", g.flag, g.country, g.city, g.subdivision),
        "yaml" => println!("---\nflag: {}\ncountry: {}\ncity: {}\nsubdivision: {}\n", g.flag, g.country, g.city, g.subdivision),
        _ => println!("{}  {} -- {}, {}", g.flag, g.country, g.city, g.subdivision),
    };

    return 0;
}

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .global_setting(AppSettings::ColorAuto)
        .global_setting(AppSettings::ColoredHelp)
        .global_setting(AppSettings::DeriveDisplayOrder)
        .global_setting(AppSettings::HidePossibleValuesInHelp)
        .arg(
            Arg::with_name("IP address")
                .help("IP address for GeoIP search")
                .empty_values(false)
        )
        .arg(
            Arg::with_name("format")
                .long("format")
                .overrides_with("format")
                .takes_value(true)
                .value_name("1row|json|yaml")
                .short("f")
                .possible_values(&["1row", "json", "yaml"])
                .hide_default_value(true)
                .default_value("1row")
                .help("Available format for output")
        )
        .arg(
            Arg::with_name("quiet")
                .long("quiet")
                .overrides_with("quiet")
                .short("q")
                .help("Minimize output")
        )
        .get_matches();

    if let Some(ip) = matches.value_of("IP address") {
        let format = matches.value_of("format").unwrap();
        process::exit(run(ip, format));
    } else {
        println!("IP address is required");
        process::exit(1);
    }
}

