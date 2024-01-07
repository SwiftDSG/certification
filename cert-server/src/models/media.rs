use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fs::remove_file;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MediaExtension {
    ICO,
    SVG,
    PNG,
    JPG,
    MP4,
    WEBP,
    WEBM,
}
#[derive(Debug, MultipartForm)]
pub struct MediaMultipartRequest {
    pub file: Vec<TempFile>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Media {
    pub _id: ObjectId,
    pub extension: MediaExtension,
}
#[derive(Deserialize, Serialize, Clone)]
pub struct MediaResponse {
    pub id: String,
    pub ext: MediaExtension,
}

impl From<&str> for MediaExtension {
    fn from(a: &str) -> Self {
        match a {
            "ico" => Self::ICO,
            "svg" => Self::SVG,
            "png" => Self::PNG,
            "jpg" => Self::JPG,
            "mp4" => Self::MP4,
            "webp" => Self::WEBP,
            "webm" => Self::WEBM,
            _ => Self::MP4,
        }
    }
}
impl From<Media> for MediaResponse {
    fn from(a: Media) -> Self {
        Self {
            id: a._id.to_string(),
            ext: a.extension,
        }
    }
}

impl MediaExtension {
    pub fn to_string(&self) -> String {
        let extension = serde_json::to_string(self).unwrap();
        extension[1..=(extension.len() - 2)].to_owned()
    }
    pub fn to_string_mime(&self) -> String {
        match self {
            MediaExtension::ICO => "image/ico".to_owned(),
            MediaExtension::SVG => "image/svg+xml".to_owned(),
            MediaExtension::PNG => "image/png".to_owned(),
            MediaExtension::JPG => "image/jpeg".to_owned(),
            MediaExtension::MP4 => "video/mp4".to_owned(),
            MediaExtension::WEBP => "image/webp".to_owned(),
            MediaExtension::WEBM => "video/webm".to_owned(),
        }
    }
}
impl Media {
    pub fn new(extension: MediaExtension) -> Self {
        Self {
            _id: ObjectId::new(),
            extension,
        }
    }
    pub fn delete(&self) {
        match remove_file(format!(
            "./files/{}.{}",
            self._id.to_string(),
            self.extension.to_string()
        )) {
            _ => (),
        }
    }
}
