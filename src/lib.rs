extern crate maxminddb;
extern crate reqwest;
extern crate libflate;

mod emoji;
use std::io;
use std::fs;
use std::path::Path;
use std::net::IpAddr;
use std::str::FromStr;
use std::env;
use maxminddb::geoip2;
use libflate::gzip::Decoder;

static SOURCE: &'static str = "https://geolite.maxmind.com/download/geoip/database/GeoLite2-City.mmdb.gz";
static TEMPFILE: &'static str = "GeoLite2-City.mmdb.gz";
static FILE: &'static str = "/usr/local/share/geoip/GeoLite2-City.mmdb";
static EXPIRATION_DURATION: u64 = 3600 * 24 * 32;

#[derive(Clone, Debug)]
pub struct GeoIP {
    pub country: String,
    pub subdivision: String,
    pub city: String,
    pub flag: String,
}

impl GeoIP {
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
