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
use std::rc::Rc;

use reqwest;

use super::{configuration, urlencode, Error};

pub struct ReviewsApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl ReviewsApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> ReviewsApiClient {
        ReviewsApiClient {
            configuration: configuration,
        }
    }
}

pub trait ReviewsApi {
    fn get_review_details(&self, review_id: &str) -> Result<crate::models::Review, Error>;
}

impl ReviewsApi for ReviewsApiClient {
    fn get_review_details(&self, review_id: &str) -> Result<crate::models::Review, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/review/{review_id}",
            configuration.base_path,
            review_id = urlencode(review_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

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
