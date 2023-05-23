use std::{io, path};

use flate2::read::GzDecoder;
use reqwest;
use serde::Deserialize;
use tar::Archive;

use crate::error::Error;
use crate::utils::vale_arch;

// https://github.com/errata-ai/vale/releases/download/v2.27.0/vale_2.27.0_macOS_arm64.tar.gz
const RELEASES: &str = "https://github.com/errata-ai/vale/releases/download";
const LATEST: &str = "https://api.github.com/repos/errata-ai/vale/releases/latest";

#[derive(Deserialize, Debug)]
pub(crate) struct Release {
    tag_name: String,
}

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
    pub fn install(&self, path: &path::Path, version: &str) -> Result<(), Error> {
        let mut v = version.to_string();
        if v == "latest" {
            v = self.fetch_version()?;
        }

        let mut asset = format!("/v{}/vale_{}_{}.tar.gz", v, v, self.arch);
        if self.arch.to_lowercase().contains("windows") {
            asset = format!("/v{}/vale_{}_{}.zip", v, v, self.arch);
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

    /// `fetch_version` returns the latest version of Vale.
    fn fetch_version(&self) -> Result<String, Error> {
        let client = reqwest::blocking::Client::builder()
            .user_agent("libvale")
            .build()?;

        let resp = client.get(LATEST).send()?;
        let info: Release = resp.json()?;

        let tag = info.tag_name.strip_prefix("v").unwrap().to_string();
        Ok(tag)
    }
}
