/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 * 
 * Generated by: https://openapi-generator.tech
 */

use std::rc::Rc;
use std::borrow::Borrow;

use reqwest;

use super::{configuration, urlencode};
use crate::Error;

pub struct TVEpisodeGroupsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl TVEpisodeGroupsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> TVEpisodeGroupsApiClient {
        TVEpisodeGroupsApiClient {
            configuration,
        }
    }
}

pub trait TVEpisodeGroupsApi {
    fn get_episode_group_details(&self, episode_group_id: &str, language: Option<&str>,) -> Result<crate::models::EpisodeGroupDetails, Error>;
}

impl TVEpisodeGroupsApi for TVEpisodeGroupsApiClient {
    fn get_episode_group_details(&self, episode_group_id: &str, language: Option<&str>,) -> Result<crate::models::EpisodeGroupDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/episode_group/{episode_group_id}", configuration.base_path, episode_group_id=urlencode(episode_group_id));
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
