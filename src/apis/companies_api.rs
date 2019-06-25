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

use super::{Error, configuration};

pub struct CompaniesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl CompaniesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> CompaniesApiClient {
        CompaniesApiClient {
            configuration: configuration,
        }
    }
}

pub trait CompaniesApi {
    fn get_company_details(&self, company_id: i32) -> Result<::models::CompanyDetails, Error>;
    fn get_company_movies_paginated(&self, company_id: i32, language: &str) -> Result<::models::MoviePaginated, Error>;
}

impl CompaniesApi for CompaniesApiClient {
    fn get_company_details(&self, company_id: i32) -> Result<::models::CompanyDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/company/{company_id}", configuration.base_path, company_id=company_id);
        let mut req_builder = client.get(uri_str.as_str());

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

    fn get_company_movies_paginated(&self, company_id: i32, language: &str) -> Result<::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/company/{company_id}/movies", configuration.base_path, company_id=company_id);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("language", &language.to_string())]);
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
