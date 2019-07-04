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
pub struct PersonDetails {
    #[serde(rename = "adult", skip_serializing_if = "Option::is_none")]
    pub adult: Option<bool>,
    #[serde(rename = "also_known_as", skip_serializing_if = "Option::is_none")]
    pub also_known_as: Option<Vec<serde_json::Value>>,
    #[serde(rename = "biography", skip_serializing_if = "Option::is_none")]
    pub biography: Option<String>,
    #[serde(rename = "birthday", skip_serializing_if = "Option::is_none")]
    pub birthday: Option<String>,
    #[serde(rename = "deathday", skip_serializing_if = "Option::is_none")]
    pub deathday: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<i32>,
    #[serde(rename = "homepage", skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "imdb_id", skip_serializing_if = "Option::is_none")]
    pub imdb_id: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "place_of_birth", skip_serializing_if = "Option::is_none")]
    pub place_of_birth: Option<String>,
    #[serde(rename = "popularity", skip_serializing_if = "Option::is_none")]
    pub popularity: Option<f32>,
    #[serde(rename = "profile_path", skip_serializing_if = "Option::is_none")]
    pub profile_path: Option<String>,
}

impl PersonDetails {
    pub fn new() -> PersonDetails {
        PersonDetails {
            adult: None,
            also_known_as: None,
            biography: None,
            birthday: None,
            deathday: None,
            gender: None,
            homepage: None,
            id: None,
            imdb_id: None,
            name: None,
            place_of_birth: None,
            popularity: None,
            profile_path: None,
        }
    }
}
