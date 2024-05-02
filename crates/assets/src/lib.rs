use std::borrow::Cow;

use anyhow::anyhow;

use gpui::{AssetSource, Result, SharedString};

pub mod icons;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "./"]
#[include = "icons/**/*.svg"]

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Cow<'static, [u8]>> {
        Self::get(path)
            .map(|f| f.data)
            .ok_or_else(|| anyhow!("could not find asset at path \"{}\"", path))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| {
                if p.starts_with(path) {
                    Some(p.into())
                } else {
                    None
                }
            })
            .collect())
    }
}
