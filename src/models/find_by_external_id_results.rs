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
pub struct FindByExternalIdResults {
    #[serde(rename = "movie_results", skip_serializing_if = "Option::is_none")]
    pub movie_results: Option<Vec<crate::models::MovieObject>>,
    #[serde(rename = "person_results", skip_serializing_if = "Option::is_none")]
    pub person_results: Option<Vec<crate::models::PersonObject>>,
    #[serde(rename = "tv_results", skip_serializing_if = "Option::is_none")]
    pub tv_results: Option<Vec<crate::models::TvObject>>,
    #[serde(rename = "tv_episode_results", skip_serializing_if = "Option::is_none")]
    pub tv_episode_results: Option<Vec<String>>,
    #[serde(rename = "tv_season_results", skip_serializing_if = "Option::is_none")]
    pub tv_season_results: Option<Vec<String>>,
}

impl FindByExternalIdResults {
    pub fn new() -> FindByExternalIdResults {
        FindByExternalIdResults {
            movie_results: None,
            person_results: None,
            tv_results: None,
            tv_episode_results: None,
            tv_season_results: None,
        }
    }
}
