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

pub struct GenresApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl GenresApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> GenresApiClient {
        GenresApiClient {
            configuration,
        }
    }
}

pub trait GenresApi {
    fn get_all_movie_genres_list(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::GenresList, Error>;
    fn get_all_tv_genres_list(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::GenresList, Error>;
    fn get_movies_by_genre_paginated(
        &self,
        genre_id: i32,
        language: Option<&str>,
        include_adult: Option<bool>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
}

impl GenresApi for GenresApiClient {
    fn get_all_movie_genres_list(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::GenresList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/genre/movie/list", configuration.base_path);
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

    fn get_all_tv_genres_list(
        &self,
        language: Option<&str>,
    ) -> Result<crate::models::GenresList, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!("{}/genre/tv/list", configuration.base_path);
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

    fn get_movies_by_genre_paginated(
        &self,
        genre_id: i32,
        language: Option<&str>,
        include_adult: Option<bool>,
        sort_by: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let mut client = configuration.rate_limit_client();

        let uri_str = format!(
            "{}/genre/{genre_id}/movies",
            configuration.base_path,
            genre_id = genre_id
        );
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
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
