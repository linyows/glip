use geoip::GeoIP;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = &args[1];
    let ip: GeoIP = GeoIP::new(arg);
    println!("{}  {} -- {}, {}", ip.flag, ip.country, ip.city, ip.subdivision);
}

