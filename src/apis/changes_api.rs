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

use super::{Error, configuration, urlencode};

pub struct ChangesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ChangesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ChangesApiClient {
        ChangesApiClient {
            configuration: configuration,
        }
    }
}

pub trait ChangesApi {
    fn get_movie_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error>;
    fn get_person_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error>;
    fn get_tv_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error>;
}

impl ChangesApi for ChangesApiClient {
    fn get_movie_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/movie/changes", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("start_date", &start_date.to_string())]);
        req_builder = req_builder.query(&[("end_date", &end_date.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_person_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/changes", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("start_date", &start_date.to_string())]);
        req_builder = req_builder.query(&[("end_date", &end_date.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_tv_changes_paginated(&self, start_date: String, end_date: String) -> Result<::models::ChangesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/tv/changes", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("start_date", &start_date.to_string())]);
        req_builder = req_builder.query(&[("end_date", &end_date.to_string())]);
        if let Some(ref apikey) = configuration.api_key {
            let key = apikey.key.clone();
            let val = match apikey.prefix {
                Some(ref prefix) => format!("{} {}", prefix, key),
                None => key,
            };
            req_builder = req_builder.query(&[("api_key", val)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

}
