// geoip - Copyright 2019 Tomohisa Oda

#[macro_use]
extern crate clap;
extern crate geoip;

use clap::{App, Arg, AppSettings};
use geoip::GeoIP;
use std::process;

fn run(ipaddress: &str) {
    let ip: GeoIP = GeoIP::new(ipaddress);
    println!("{}  {} -- {}, {}", ip.flag, ip.country, ip.city, ip.subdivision);
    process::exit(0);
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

    if let Some(ipaddress) = matches.value_of("IP address") {
        run(ipaddress);
    } else {
        println!("IP address is required");
        process::exit(1);
    }
}

