/*
 * API
 *
 * ## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description
 *
 * The version of the OpenAPI document: 3
 *
 * Generated by: https://openapi-generator.tech
 */

use std::borrow::Borrow;
use std::option::Option;
use std::rc::Rc;

use reqwest;

use super::{configuration, urlencode};
use crate::Error;

pub struct GuestSessionsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl GuestSessionsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> GuestSessionsApiClient {
        GuestSessionsApiClient {
            configuration,
        }
    }
}

pub trait GuestSessionsApi {
    fn get_guest_session_rated_movies_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_guest_session_rated_tv_episodes_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvEpisodesPaginated, Error>;
    fn get_guest_session_rated_tv_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error>;
}

impl GuestSessionsApi for GuestSessionsApiClient {
    fn get_guest_session_rated_movies_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/guest_session/{guest_session_id}/rated/movies",
            configuration.base_path,
            guest_session_id = urlencode(guest_session_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_guest_session_rated_tv_episodes_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvEpisodesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/guest_session/{guest_session_id}/rated/tv/episodes",
            configuration.base_path,
            guest_session_id = urlencode(guest_session_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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

    fn get_guest_session_rated_tv_paginated(
        &self,
        guest_session_id: &str,
        language: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/guest_session/{guest_session_id}/rated/tv",
            configuration.base_path,
            guest_session_id = urlencode(guest_session_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
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
