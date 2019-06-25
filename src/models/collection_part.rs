/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CollectionPart {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "genre_ids", skip_serializing_if = "Option::is_none")]
    pub genre_ids: Option<Vec<i32>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "original_language", skip_serializing_if = "Option::is_none")]
    pub original_language: Option<String>,
    #[serde(rename = "original_title", skip_serializing_if = "Option::is_none")]
    pub original_title: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "release_date", skip_serializing_if = "Option::is_none")]
    pub release_date: Option<String>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<bool>,
    #[serde(rename = "vote_average", skip_serializing_if = "Option::is_none")]
    pub vote_average: Option<f32>,
    #[serde(rename = "vote_count", skip_serializing_if = "Option::is_none")]
    pub vote_count: Option<i32>,
}

impl CollectionPart {
    pub fn new() -> CollectionPart {
        CollectionPart {
            adult: None,
            genre_ids: None,
            id: None,
            original_language: None,
            original_title: None,
            overview: None,
            release_date: None,
            poster_path: None,
            popularity: None,
            title: None,
            video: None,
            vote_average: None,
            vote_count: None,
        }
    }
}
