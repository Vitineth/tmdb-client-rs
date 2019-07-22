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

use super::{configuration, urlencode, Error};

pub struct MoviesApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl MoviesApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> MoviesApiClient {
        MoviesApiClient {
            configuration: configuration,
        }
    }
}

pub trait MoviesApi {
    fn delete_movie_rating(
        &self,
        movie_id: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::StatusCodeMsgResponse, Error>;
    fn get_movie_account_states(
        &self,
        movie_id: &str,
        session_id: Option<&str>,
        guest_session_id: Option<&str>,
    ) -> Result<crate::models::AccountStates, Error>;
    fn get_movie_alternative_titles_list(
        &self,
        movie_id: i32,
        country: Option<&str>,
    ) -> Result<crate::models::AlternativeTitlesList, Error>;
    fn get_movie_changes_list(
        &self,
        movie_id: &str,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error>;
    fn get_movie_credits(&self, movie_id: i32) -> Result<crate::models::Credits, Error>;
    fn get_movie_external_ids(&self, movie_id: i32) -> Result<crate::models::ExternalIds, Error>;
    fn get_movie_details(
        &self,
        movie_id: i32,
        language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::MovieDetails, Error>;
    fn get_movie_images(
        &self,
        movie_id: i32,
        include_image_language: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::Images, Error>;
    fn get_movie_keywords_list(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::KeywordsList, Error>;
    fn get_movie_latest_details(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::MovieDetails, Error>;
    fn get_movie_lists_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::ListsPaginated, Error>;
    fn get_movie_now_playing_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_popular_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_recommendations_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_release_dates(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::ReleaseDatesList, Error>;
    fn get_movie_reviews_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::ReviewsPaginated, Error>;
    fn get_movie_similar_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_top_rated_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_translations(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error>;
    fn get_movie_upcoming_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_movie_videos_list(
        &self,
        movie_id: &str,
        api_key: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error>;
    fn post_movie_rating(
        &self,
        movie_id: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
        body: Option<crate::models::ValueBody>,
    ) -> Result<crate::models::StatusCodeMsgResponse, Error>;
}

impl MoviesApi for MoviesApiClient {
    fn delete_movie_rating(
        &self,
        movie_id: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
    ) -> Result<crate::models::StatusCodeMsgResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/rating",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.delete(uri_str.as_str());

        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }

    fn get_movie_account_states(
        &self,
        movie_id: &str,
        session_id: Option<&str>,
        guest_session_id: Option<&str>,
    ) -> Result<crate::models::AccountStates, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/account_states",
            configuration.base_path,
            movie_id = urlencode(movie_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
        }
        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
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

    fn get_movie_alternative_titles_list(
        &self,
        movie_id: i32,
        country: Option<&str>,
    ) -> Result<crate::models::AlternativeTitlesList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/alternative_titles",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = country {
            req_builder = req_builder.query(&[("country", &s.to_string())]);
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

    fn get_movie_changes_list(
        &self,
        movie_id: &str,
        start_date: Option<String>,
        end_date: Option<String>,
        page: Option<i32>,
    ) -> Result<crate::models::ChangeDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/changes",
            configuration.base_path,
            movie_id = urlencode(movie_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

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

    fn get_movie_credits(&self, movie_id: i32) -> Result<crate::models::Credits, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/credits",
            configuration.base_path,
            movie_id = movie_id
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

    fn get_movie_external_ids(&self, movie_id: i32) -> Result<crate::models::ExternalIds, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/external_ids",
            configuration.base_path,
            movie_id = movie_id
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

    fn get_movie_details(
        &self,
        movie_id: i32,
        language: Option<&str>,
        append_to_response: Option<&str>,
    ) -> Result<crate::models::MovieDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
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

    fn get_movie_images(
        &self,
        movie_id: i32,
        include_image_language: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::Images, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/images",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = include_image_language {
            req_builder = req_builder.query(&[("include_image_language", &s.to_string())]);
        }
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

    fn get_movie_keywords_list(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::KeywordsList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/keywords",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
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

    fn get_movie_latest_details(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::MovieDetails, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/movie/latest", configuration.base_path);
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

    fn get_movie_lists_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::ListsPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/lists",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
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

    fn get_movie_now_playing_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/movie/now_playing", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_movie_popular_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/movie/popular", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_movie_recommendations_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/recommendations",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
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

    fn get_movie_release_dates(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::ReleaseDatesList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/release_dates",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
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

    fn get_movie_reviews_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::ReviewsPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/reviews",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
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

    fn get_movie_similar_paginated(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
        language: Option<&str>,
        page: Option<i32>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/similar",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
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

    fn get_movie_top_rated_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/movie/top_rated", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_movie_translations(
        &self,
        movie_id: i32,
        api_key: Option<&str>,
    ) -> Result<crate::models::TranslationsList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/translations",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
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

    fn get_movie_upcoming_paginated(
        &self,
        language: Option<&str>,
        page: Option<i32>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/movie/upcoming", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = region {
            req_builder = req_builder.query(&[("region", &s.to_string())]);
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

    fn get_movie_videos_list(
        &self,
        movie_id: &str,
        api_key: Option<&str>,
        language: Option<&str>,
    ) -> Result<crate::models::VideosList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/videos",
            configuration.base_path,
            movie_id = urlencode(movie_id)
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = api_key {
            req_builder = req_builder.query(&[("api_key", &s.to_string())]);
        }
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

    fn post_movie_rating(
        &self,
        movie_id: i32,
        content_type: &str,
        guest_session_id: Option<&str>,
        session_id: Option<&str>,
        body: Option<crate::models::ValueBody>,
    ) -> Result<crate::models::StatusCodeMsgResponse, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/movie/{movie_id}/rating",
            configuration.base_path,
            movie_id = movie_id
        );
        let mut req_builder = client.post(uri_str.as_str());

        if let Some(ref s) = guest_session_id {
            req_builder = req_builder.query(&[("guest_session_id", &s.to_string())]);
        }
        if let Some(ref s) = session_id {
            req_builder = req_builder.query(&[("session_id", &s.to_string())]);
        }
        if let Some(ref apikey) = configuration.api_key {
            req_builder = req_builder.query(&[("api_key", apikey)]);
        }
        if let Some(ref user_agent) = configuration.user_agent {
            req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        req_builder = req_builder.header("Content-Type", content_type.to_string());
        req_builder = req_builder.json(&body);

        // send request
        let req = req_builder.build()?;

        Ok(client.execute(req)?.error_for_status()?.json()?)
    }
}
