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

pub struct DiscoverApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DiscoverApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DiscoverApiClient {
        DiscoverApiClient {
            configuration,
        }
    }
}

pub trait DiscoverApi {
    fn get_discover_movie_paginated(
        &self,
        sort_by: Option<&str>,
        certification_country: Option<&str>,
        certification: Option<&str>,
        certification_lte: Option<&str>,
        include_adult: Option<bool>,
        include_video: Option<bool>,
        language: Option<&str>,
        page: Option<i32>,
        primary_release_year: Option<i32>,
        primary_release_date_gte: Option<String>,
        primary_release_date_lte: Option<String>,
        release_date_gte: Option<String>,
        release_date_lte: Option<String>,
        vote_count_gte: Option<i32>,
        vote_count_lte: Option<i32>,
        vote_average_gte: Option<f32>,
        vote_average_lte: Option<f32>,
        with_cast: Option<&str>,
        with_crew: Option<&str>,
        with_companies: Option<&str>,
        with_genres: Option<&str>,
        with_keywords: Option<&str>,
        with_people: Option<&str>,
        year: Option<i32>,
        without_genres: Option<&str>,
        with_runtime_gte: Option<i32>,
        with_runtime_lte: Option<i32>,
        with_release_type: Option<i32>,
        with_original_language: Option<&str>,
        without_keywords: Option<&str>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error>;
    fn get_discover_tv_paginated(
        &self,
        sort_by: Option<&str>,
        air_date_gte: Option<String>,
        air_date_lte: Option<String>,
        first_air_date_gte: Option<String>,
        first_air_date_lte: Option<String>,
        first_air_date_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
        timezone: Option<&str>,
        vote_average_gte: Option<f32>,
        vote_count_gte: Option<i32>,
        with_genres: Option<&str>,
        with_networks: Option<&str>,
        without_genres: Option<&str>,
        with_runtime_gte: Option<i32>,
        with_runtime_lte: Option<i32>,
        include_null_first_air_dates: Option<bool>,
        with_original_language: Option<&str>,
        without_keywords: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error>;
}

impl DiscoverApi for DiscoverApiClient {
    fn get_discover_movie_paginated(
        &self,
        sort_by: Option<&str>,
        certification_country: Option<&str>,
        certification: Option<&str>,
        certification_lte: Option<&str>,
        include_adult: Option<bool>,
        include_video: Option<bool>,
        language: Option<&str>,
        page: Option<i32>,
        primary_release_year: Option<i32>,
        primary_release_date_gte: Option<String>,
        primary_release_date_lte: Option<String>,
        release_date_gte: Option<String>,
        release_date_lte: Option<String>,
        vote_count_gte: Option<i32>,
        vote_count_lte: Option<i32>,
        vote_average_gte: Option<f32>,
        vote_average_lte: Option<f32>,
        with_cast: Option<&str>,
        with_crew: Option<&str>,
        with_companies: Option<&str>,
        with_genres: Option<&str>,
        with_keywords: Option<&str>,
        with_people: Option<&str>,
        year: Option<i32>,
        without_genres: Option<&str>,
        with_runtime_gte: Option<i32>,
        with_runtime_lte: Option<i32>,
        with_release_type: Option<i32>,
        with_original_language: Option<&str>,
        without_keywords: Option<&str>,
        region: Option<&str>,
    ) -> Result<crate::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/discover/movie", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
        }
        if let Some(ref s) = certification_country {
            req_builder = req_builder.query(&[("certification_country", &s.to_string())]);
        }
        if let Some(ref s) = certification {
            req_builder = req_builder.query(&[("certification", &s.to_string())]);
        }
        if let Some(ref s) = certification_lte {
            req_builder = req_builder.query(&[("certification.lte", &s.to_string())]);
        }
        if let Some(ref s) = include_adult {
            req_builder = req_builder.query(&[("include_adult", &s.to_string())]);
        }
        if let Some(ref s) = include_video {
            req_builder = req_builder.query(&[("include_video", &s.to_string())]);
        }
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = primary_release_year {
            req_builder = req_builder.query(&[("primary_release_year", &s.to_string())]);
        }
        if let Some(ref s) = primary_release_date_gte {
            req_builder = req_builder.query(&[("primary_release_date.gte", &s.to_string())]);
        }
        if let Some(ref s) = primary_release_date_lte {
            req_builder = req_builder.query(&[("primary_release_date.lte", &s.to_string())]);
        }
        if let Some(ref s) = release_date_gte {
            req_builder = req_builder.query(&[("release_date.gte", &s.to_string())]);
        }
        if let Some(ref s) = release_date_lte {
            req_builder = req_builder.query(&[("release_date.lte", &s.to_string())]);
        }
        if let Some(ref s) = vote_count_gte {
            req_builder = req_builder.query(&[("vote_count.gte", &s.to_string())]);
        }
        if let Some(ref s) = vote_count_lte {
            req_builder = req_builder.query(&[("vote_count.lte", &s.to_string())]);
        }
        if let Some(ref s) = vote_average_gte {
            req_builder = req_builder.query(&[("vote_average.gte", &s.to_string())]);
        }
        if let Some(ref s) = vote_average_lte {
            req_builder = req_builder.query(&[("vote_average.lte", &s.to_string())]);
        }
        if let Some(ref s) = with_cast {
            req_builder = req_builder.query(&[("with_cast", &s.to_string())]);
        }
        if let Some(ref s) = with_crew {
            req_builder = req_builder.query(&[("with_crew", &s.to_string())]);
        }
        if let Some(ref s) = with_companies {
            req_builder = req_builder.query(&[("with_companies", &s.to_string())]);
        }
        if let Some(ref s) = with_genres {
            req_builder = req_builder.query(&[("with_genres", &s.to_string())]);
        }
        if let Some(ref s) = with_keywords {
            req_builder = req_builder.query(&[("with_keywords", &s.to_string())]);
        }
        if let Some(ref s) = with_people {
            req_builder = req_builder.query(&[("with_people", &s.to_string())]);
        }
        if let Some(ref s) = year {
            req_builder = req_builder.query(&[("year", &s.to_string())]);
        }
        if let Some(ref s) = without_genres {
            req_builder = req_builder.query(&[("without_genres", &s.to_string())]);
        }
        if let Some(ref s) = with_runtime_gte {
            req_builder = req_builder.query(&[("with_runtime.gte", &s.to_string())]);
        }
        if let Some(ref s) = with_runtime_lte {
            req_builder = req_builder.query(&[("with_runtime.lte", &s.to_string())]);
        }
        if let Some(ref s) = with_release_type {
            req_builder = req_builder.query(&[("with_release_type", &s.to_string())]);
        }
        if let Some(ref s) = with_original_language {
            req_builder = req_builder.query(&[("with_original_language", &s.to_string())]);
        }
        if let Some(ref s) = without_keywords {
            req_builder = req_builder.query(&[("without_keywords", &s.to_string())]);
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

    fn get_discover_tv_paginated(
        &self,
        sort_by: Option<&str>,
        air_date_gte: Option<String>,
        air_date_lte: Option<String>,
        first_air_date_gte: Option<String>,
        first_air_date_lte: Option<String>,
        first_air_date_year: Option<i32>,
        language: Option<&str>,
        page: Option<i32>,
        timezone: Option<&str>,
        vote_average_gte: Option<f32>,
        vote_count_gte: Option<i32>,
        with_genres: Option<&str>,
        with_networks: Option<&str>,
        without_genres: Option<&str>,
        with_runtime_gte: Option<i32>,
        with_runtime_lte: Option<i32>,
        include_null_first_air_dates: Option<bool>,
        with_original_language: Option<&str>,
        without_keywords: Option<&str>,
    ) -> Result<crate::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/discover/tv", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        if let Some(ref s) = sort_by {
            req_builder = req_builder.query(&[("sort_by", &s.to_string())]);
        }
        if let Some(ref s) = air_date_gte {
            req_builder = req_builder.query(&[("air_date.gte", &s.to_string())]);
        }
        if let Some(ref s) = air_date_lte {
            req_builder = req_builder.query(&[("air_date.lte", &s.to_string())]);
        }
        if let Some(ref s) = first_air_date_gte {
            req_builder = req_builder.query(&[("first_air_date.gte", &s.to_string())]);
        }
        if let Some(ref s) = first_air_date_lte {
            req_builder = req_builder.query(&[("first_air_date.lte", &s.to_string())]);
        }
        if let Some(ref s) = first_air_date_year {
            req_builder = req_builder.query(&[("first_air_date_year", &s.to_string())]);
        }
        if let Some(ref s) = language {
            req_builder = req_builder.query(&[("language", &s.to_string())]);
        }
        if let Some(ref s) = page {
            req_builder = req_builder.query(&[("page", &s.to_string())]);
        }
        if let Some(ref s) = timezone {
            req_builder = req_builder.query(&[("timezone", &s.to_string())]);
        }
        if let Some(ref s) = vote_average_gte {
            req_builder = req_builder.query(&[("vote_average.gte", &s.to_string())]);
        }
        if let Some(ref s) = vote_count_gte {
            req_builder = req_builder.query(&[("vote_count.gte", &s.to_string())]);
        }
        if let Some(ref s) = with_genres {
            req_builder = req_builder.query(&[("with_genres", &s.to_string())]);
        }
        if let Some(ref s) = with_networks {
            req_builder = req_builder.query(&[("with_networks", &s.to_string())]);
        }
        if let Some(ref s) = without_genres {
            req_builder = req_builder.query(&[("without_genres", &s.to_string())]);
        }
        if let Some(ref s) = with_runtime_gte {
            req_builder = req_builder.query(&[("with_runtime.gte", &s.to_string())]);
        }
        if let Some(ref s) = with_runtime_lte {
            req_builder = req_builder.query(&[("with_runtime.lte", &s.to_string())]);
        }
        if let Some(ref s) = include_null_first_air_dates {
            req_builder = req_builder.query(&[("include_null_first_air_dates", &s.to_string())]);
        }
        if let Some(ref s) = with_original_language {
            req_builder = req_builder.query(&[("with_original_language", &s.to_string())]);
        }
        if let Some(ref s) = without_keywords {
            req_builder = req_builder.query(&[("without_keywords", &s.to_string())]);
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
