use std::{io, path};

use flate2::read::GzDecoder;
use reqwest;
use tar::Archive;

use crate::error::Error;
use crate::utils::vale_arch;

const RELEASES: &str = "https://github.com/errata-ai/vale/releases/download";

#[derive(Debug, Clone)]
pub struct ValeManager {
    pub arch: String,
}

// ValeManager manages the installation and execution of Vale.
//
// ValeManager is responsible for downloading and installing Vale, as well as
// running Vale and parsing its output.
impl ValeManager {
    // `new` creates a new ValeManager.
    //
    // The ValeManager will attempt to use the managed version of Vale, but
    // will fall back to the system version if it's not available.
    pub fn new() -> ValeManager {
        let arch = vale_arch();
        ValeManager { arch }
    }

    /// `install` downloads the latest version of Vale and extracts it to the
    /// specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - A path to the directory where Vale should be installed.
    /// * `version` - A string representing the version to be installed.
    /// * `arch` - A string representing the architecture to be installed.
    pub fn install(&self, path: &path::Path, v: &str, arch: &str) -> Result<(), Error> {
        let mut asset = format!("/v{}/vale_{}_{}.tar.gz", v, v, arch);
        if arch.to_lowercase().contains("windows") {
            asset = format!("/v{}/vale_{}_{}.zip", v, v, arch);
        }
        let url = format!("{}{}", RELEASES, asset);

        let resp = reqwest::blocking::get(url)?.bytes()?;
        let archive = resp.to_vec();

        let buf = io::Cursor::new(archive);
        if asset.ends_with(".zip") {
            zip_extract::extract(buf, path, true)?;
        } else {
            Archive::new(GzDecoder::new(buf)).unpack(path)?;
        }

        Ok(())
    }
}
