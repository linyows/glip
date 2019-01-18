extern crate maxminddb;
extern crate reqwest;

mod emoji;
use std::io;
use std::fs;
use std::path::Path;
use std::net::IpAddr;
use std::str::FromStr;
use std::env;
use maxminddb::geoip2;

static SOURCE: &'static str = "https://geolite.maxmind.com/download/geoip/database/GeoLite2-City.tar.gz";
static TEMPFILE: &'static str = "geoip-db.tar.gz";
static FILE: &'static str = "/usr/local/share/GeoIP/GeoLite2-City.mmdb";
static ELAPSE_MAX: u64 = 3600 * 24 * 32;

#[derive(Clone, Debug)]
pub struct GeoIP {
    pub country: String,
    pub subdivision: String,
    pub city: String,
    pub flag: String,
}

impl GeoIP {
    fn download_file() -> () {
        let path = Path::new(FILE);
        let pdir = path.parent().unwrap();
        if !Path::new(pdir).exists() {
            fs::File::create(pdir).expect("failed to create dir");
        }
        let mut resp = reqwest::get(SOURCE).expect("download failed");
        let mut dir = env::temp_dir();
        dir.push(TEMPFILE);
        let mut out = fs::File::create(dir).expect("failed to create file");
        io::copy(&mut resp, &mut out).expect("failed to copy content");
        println!("{:?}", out);
    }

    fn is_old_file() -> bool {
        let metadata = fs::metadata(FILE);
        let modified = metadata.unwrap().modified().unwrap();
        let elapsed = modified.elapsed().unwrap();
        return elapsed.as_secs() >= ELAPSE_MAX;
    }

    fn download_file_if_not_exists_or_old() -> () {
        let exists = Path::new(FILE).exists();
        if !exists || (exists && Self::is_old_file()) {
            Self::download_file();
        }
    }

    fn reader() -> Result<maxminddb::Reader<Vec<u8>>, maxminddb::MaxMindDBError> {
        Self::download_file_if_not_exists_or_old();
        let rr = maxminddb::Reader::open_readfile(FILE);
        if let Err(err) = rr {
            panic!(format!("error opening mmdb: {:?}", err));
        }
        rr
    }

    pub fn new(ip: &str) -> Self {
        let ipaddr: IpAddr = FromStr::from_str(ip).unwrap();
        let geoip: geoip2::City = Self::reader().unwrap().lookup(ipaddr).unwrap();
        let country = geoip.country.unwrap().names.unwrap();
        let c = country.get("en").unwrap();
        let mut subdivs = geoip.subdivisions.unwrap();
        let subdiv = subdivs.pop().unwrap().names.unwrap();
        let city = geoip.city.unwrap().names.unwrap();

        GeoIP {
            country: c.to_string(),
            subdivision: subdiv.get("en").unwrap().to_string(),
            city: city.get("en").unwrap().to_string(),
            flag: emoji::flag(c),
        }
    }
}

