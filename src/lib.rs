// glip - Copyright 2019 Tomohisa Oda

extern crate maxminddb;
extern crate reqwest;
extern crate libflate;
extern crate failure;

mod emoji;
use std::io;
use std::fs;
use std::env;
use std::path::Path;
use std::net::IpAddr;
use std::str::FromStr;
use maxminddb::geoip2;
use libflate::gzip::Decoder;

static SOURCE: &'static str = "https://geolite.maxmind.com/download/geoip/database/GeoLite2-City.mmdb.gz";
static TEMPFILE: &'static str = "GeoLite2-City.mmdb.gz";
static FILE: &'static str = "/usr/local/share/glip/GeoLite2-City.mmdb";
static EXPIRATION_DURATION: u64 = 3600 * 24 * 32;
static EMPTY_TXT: &'static str = "unknown";

#[derive(Clone, Debug)]
pub struct GLIP {
    pub country: String,
    pub subdivision: String,
    pub city: String,
    pub flag: String,
}

impl GLIP {
    fn extract_file(fname: &str) {
        let mut file = fs::File::open(fname).unwrap();
        let mut decoder = Decoder::new(&mut file).unwrap();
        let mut out = fs::File::create(FILE).expect("failed to create file");
        io::copy(&mut decoder, &mut out).expect("failed to copy content");
    }

    fn download_file(fname: &str) {
        let path = Path::new(FILE);
        let pdir = path.parent().unwrap();
        if !Path::new(pdir).exists() {
            fs::create_dir_all(pdir).expect("failed to create dir");
        }
        let mut resp = reqwest::get(SOURCE).expect("download failed");
        let mut out: fs::File = fs::File::create(fname).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");
    }

    fn file_expired() -> bool {
        let metadata = fs::metadata(FILE);
        let modified = metadata.unwrap().modified().unwrap();
        let elapsed = modified.elapsed().unwrap();
        return elapsed.as_secs() >= EXPIRATION_DURATION;
    }

    fn download_and_extract_file_if_not_exists_or_old() {
        if Path::new(FILE).exists() && !Self::file_expired() {
            return
        }
        let mut dir = env::temp_dir();
        dir.push(TEMPFILE);
        let fname = dir.as_path().to_str().unwrap();
        println!("==> Database downloading...");
        Self::download_file(fname);
        println!("    Saved to {}", fname);
        println!("==> Database extracting...");
        Self::extract_file(fname);
        println!("    Extracted to {}", FILE);
    }

    fn reader() -> Result<maxminddb::Reader<Vec<u8>>, maxminddb::MaxMindDBError> {
        Self::download_and_extract_file_if_not_exists_or_old();
        let rr = maxminddb::Reader::open_readfile(FILE);
        if let Err(err) = rr {
            panic!(format!("error opening mmdb: {:?}", err));
        }
        rr
    }

    fn pickup_subdivision(src: Option<Vec<geoip2::model::Subdivision>>) -> String {
        if src.is_none() {
            return EMPTY_TXT.to_string();
        }
        let mut subdivs = src.unwrap();
        let subdiv_model = subdivs.pop().unwrap().names.unwrap();
        return subdiv_model.get("en").unwrap().to_string();
    }

    fn pickup_city(src: Option<geoip2::model::City>) -> String {
        if src.is_none() {
            return EMPTY_TXT.to_string();
        }
        let city_model = src.unwrap().names.unwrap();
        return city_model.get("en").unwrap().to_string();
    }

    pub fn new(ip: &str) -> Result<Self, failure::Error> {
        let ipaddr: IpAddr = FromStr::from_str(ip)?;
        let geoip: geoip2::City = Self::reader().unwrap().lookup(ipaddr)?;
        let country_model = geoip.country.unwrap().names.unwrap();
        let country = country_model.get("en").unwrap();

        Ok(GLIP {
            country: country.to_string(),
            subdivision: Self::pickup_subdivision(geoip.subdivisions),
            city: Self::pickup_city(geoip.city),
            flag: emoji::flag(country),
        })
    }
}
