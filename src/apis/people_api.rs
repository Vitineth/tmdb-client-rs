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

use super::configuration;
use crate::Error;

pub struct PeopleApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl PeopleApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> PeopleApiClient {
        PeopleApiClient {
            configuration,
        }
    }
}

pub trait PeopleApi {
    fn get_person_changes(
        &self,
        person_id: i32,
        language: Option<&str>,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error>;
    fn get_person_combined_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error>;
    fn get_person_details(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::PersonDetails, Error>;
    fn get_person_external_ids(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::ExternalIds, Error>;
    fn get_person_images(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error>;
    fn get_person_latest_details(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::PersonDetails, Error>;
    fn get_person_movie_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error>;
    fn get_person_popular_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::PersonPaginated, Error>;
    fn get_person_tagged_images_paginated(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::PersonTaggedImagesPaginated, Error>;
    fn get_person_translations_list(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error>;
    fn get_person_tv_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error>;
}

impl PeopleApi for PeopleApiClient {
    fn get_person_changes(
        &self,
        person_id: i32,
        language: Option<&str>,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/changes",
            configuration.base_path,
            person_id = person_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = start_date {
            req_builder = req_builder.query(&[("start_date", &s.to_string())]);
        }
        if let Some(ref s) = end_date {
            req_builder = req_builder.query(&[("end_date", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_person_combined_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/combined_credits",
            configuration.base_path,
            person_id = person_id
        );
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

    fn get_person_details(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::PersonDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}",
            configuration.base_path,
            person_id = person_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
        if let Some(ref s) = append_to_response {
            req_builder = req_builder.query(&[("append_to_response", &s.to_string())]);
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

    fn get_person_external_ids(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::ExternalIds, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/external_ids",
            configuration.base_path,
            person_id = person_id
        );
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

    fn get_person_images(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
    ) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/images",
            configuration.base_path,
            person_id = person_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
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

    fn get_person_latest_details(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::PersonDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/latest", configuration.base_path);
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

    fn get_person_movie_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/movie_credits",
            configuration.base_path,
            person_id = person_id
        );
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

    fn get_person_popular_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::PersonPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/person/popular", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_person_tagged_images_paginated(
        &self,
        person_id: i32,
        language: Option<&str>,
        include_image_language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::PersonTaggedImagesPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/tagged_images",
            configuration.base_path,
            person_id = person_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
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

    fn get_person_translations_list(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/translations",
            configuration.base_path,
            person_id = person_id
        );
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

    fn get_person_tv_credits(
        &self,
        person_id: i32,
        language: Option<&str>,
    ) -> Result<crate::models::PersonCredits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!(
            "{}/person/{person_id}/tv_credits",
            configuration.base_path,
            person_id = person_id
        );
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
