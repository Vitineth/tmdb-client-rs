/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonDetails {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<String>,
    #[serde(rename = "air_date", skip_serializing_if = "Option::is_none")]
    pub air_date: Option<String>,
    #[serde(rename = "episodes", skip_serializing_if = "Option::is_none")]
    pub episodes: Option<Vec<crate::models::EpisodeDetails>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "overview", skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "poster_path", skip_serializing_if = "Option::is_none")]
    pub poster_path: Option<String>,
    #[serde(rename = "season_number", skip_serializing_if = "Option::is_none")]
    pub season_number: Option<i32>,
}

impl SeasonDetails {
    pub fn new() -> SeasonDetails {
        SeasonDetails {
            _id: None,
            air_date: None,
            episodes: None,
            name: None,
            overview: None,
            id: None,
            poster_path: None,
            season_number: None,
        }
    }
}
