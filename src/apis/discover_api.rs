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

pub struct DiscoverApiClient {
    configuration: Rc<configuration::Configuration>,
}

impl DiscoverApiClient {
    pub fn new(configuration: Rc<configuration::Configuration>) -> DiscoverApiClient {
        DiscoverApiClient {
            configuration: configuration,
        }
    }
}

pub trait DiscoverApi {
    fn get_discover_movie_paginated(&self, sort_by: &str, certification_country: &str, certification: &str, certification_lte: &str, include_adult: bool, include_video: bool, language: &str, page: i32, primary_release_year: i32, primary_release_date_gte: String, primary_release_date_lte: String, release_date_gte: String, release_date_lte: String, vote_count_gte: i32, vote_count_lte: i32, vote_average_gte: f32, vote_average_lte: f32, with_cast: &str, with_crew: &str, with_companies: &str, with_genres: &str, with_keywords: &str, with_people: &str, year: i32, without_genres: &str, with_runtime_gte: i32, with_runtime_lte: i32, with_release_type: i32, with_original_language: &str, without_keywords: &str, region: &str) -> Result<::models::MoviePaginated, Error>;
    fn get_discover_tv_paginated(&self, sort_by: &str, air_date_gte: String, air_date_lte: String, first_air_date_gte: String, first_air_date_lte: String, first_air_date_year: i32, language: &str, page: i32, timezone: &str, vote_average_gte: f32, vote_count_gte: i32, with_genres: &str, with_networks: &str, without_genres: &str, with_runtime_gte: i32, with_runtime_lte: i32, include_null_first_air_dates: bool, with_original_language: &str, without_keywords: &str) -> Result<::models::TvPaginated, Error>;
}

impl DiscoverApi for DiscoverApiClient {
    fn get_discover_movie_paginated(&self, sort_by: &str, certification_country: &str, certification: &str, certification_lte: &str, include_adult: bool, include_video: bool, language: &str, page: i32, primary_release_year: i32, primary_release_date_gte: String, primary_release_date_lte: String, release_date_gte: String, release_date_lte: String, vote_count_gte: i32, vote_count_lte: i32, vote_average_gte: f32, vote_average_lte: f32, with_cast: &str, with_crew: &str, with_companies: &str, with_genres: &str, with_keywords: &str, with_people: &str, year: i32, without_genres: &str, with_runtime_gte: i32, with_runtime_lte: i32, with_release_type: i32, with_original_language: &str, without_keywords: &str, region: &str) -> Result<::models::MoviePaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/discover/movie", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("sort_by", &sort_by.to_string())]);
        req_builder = req_builder.query(&[("certification_country", &certification_country.to_string())]);
        req_builder = req_builder.query(&[("certification", &certification.to_string())]);
        req_builder = req_builder.query(&[("certification.lte", &certification_lte.to_string())]);
        req_builder = req_builder.query(&[("include_adult", &include_adult.to_string())]);
        req_builder = req_builder.query(&[("include_video", &include_video.to_string())]);
        req_builder = req_builder.query(&[("language", &language.to_string())]);
        req_builder = req_builder.query(&[("page", &page.to_string())]);
        req_builder = req_builder.query(&[("primary_release_year", &primary_release_year.to_string())]);
        req_builder = req_builder.query(&[("primary_release_date.gte", &primary_release_date_gte.to_string())]);
        req_builder = req_builder.query(&[("primary_release_date.lte", &primary_release_date_lte.to_string())]);
        req_builder = req_builder.query(&[("release_date.gte", &release_date_gte.to_string())]);
        req_builder = req_builder.query(&[("release_date.lte", &release_date_lte.to_string())]);
        req_builder = req_builder.query(&[("vote_count.gte", &vote_count_gte.to_string())]);
        req_builder = req_builder.query(&[("vote_count.lte", &vote_count_lte.to_string())]);
        req_builder = req_builder.query(&[("vote_average.gte", &vote_average_gte.to_string())]);
        req_builder = req_builder.query(&[("vote_average.lte", &vote_average_lte.to_string())]);
        req_builder = req_builder.query(&[("with_cast", &with_cast.to_string())]);
        req_builder = req_builder.query(&[("with_crew", &with_crew.to_string())]);
        req_builder = req_builder.query(&[("with_companies", &with_companies.to_string())]);
        req_builder = req_builder.query(&[("with_genres", &with_genres.to_string())]);
        req_builder = req_builder.query(&[("with_keywords", &with_keywords.to_string())]);
        req_builder = req_builder.query(&[("with_people", &with_people.to_string())]);
        req_builder = req_builder.query(&[("year", &year.to_string())]);
        req_builder = req_builder.query(&[("without_genres", &without_genres.to_string())]);
        req_builder = req_builder.query(&[("with_runtime.gte", &with_runtime_gte.to_string())]);
        req_builder = req_builder.query(&[("with_runtime.lte", &with_runtime_lte.to_string())]);
        req_builder = req_builder.query(&[("with_release_type", &with_release_type.to_string())]);
        req_builder = req_builder.query(&[("with_original_language", &with_original_language.to_string())]);
        req_builder = req_builder.query(&[("without_keywords", &without_keywords.to_string())]);
        req_builder = req_builder.query(&[("region", &region.to_string())]);
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

    fn get_discover_tv_paginated(&self, sort_by: &str, air_date_gte: String, air_date_lte: String, first_air_date_gte: String, first_air_date_lte: String, first_air_date_year: i32, language: &str, page: i32, timezone: &str, vote_average_gte: f32, vote_count_gte: i32, with_genres: &str, with_networks: &str, without_genres: &str, with_runtime_gte: i32, with_runtime_lte: i32, include_null_first_air_dates: bool, with_original_language: &str, without_keywords: &str) -> Result<::models::TvPaginated, Error> {
        let configuration: &configuration::Configuration = self.configuration.borrow();
        let client = &configuration.client;

        let uri_str = format!("{}/discover/tv", configuration.base_path);
        let mut req_builder = client.get(uri_str.as_str());

        req_builder = req_builder.query(&[("sort_by", &sort_by.to_string())]);
        req_builder = req_builder.query(&[("air_date.gte", &air_date_gte.to_string())]);
        req_builder = req_builder.query(&[("air_date.lte", &air_date_lte.to_string())]);
        req_builder = req_builder.query(&[("first_air_date.gte", &first_air_date_gte.to_string())]);
        req_builder = req_builder.query(&[("first_air_date.lte", &first_air_date_lte.to_string())]);
        req_builder = req_builder.query(&[("first_air_date_year", &first_air_date_year.to_string())]);
        req_builder = req_builder.query(&[("language", &language.to_string())]);
        req_builder = req_builder.query(&[("page", &page.to_string())]);
        req_builder = req_builder.query(&[("timezone", &timezone.to_string())]);
        req_builder = req_builder.query(&[("vote_average.gte", &vote_average_gte.to_string())]);
        req_builder = req_builder.query(&[("vote_count.gte", &vote_count_gte.to_string())]);
        req_builder = req_builder.query(&[("with_genres", &with_genres.to_string())]);
        req_builder = req_builder.query(&[("with_networks", &with_networks.to_string())]);
        req_builder = req_builder.query(&[("without_genres", &without_genres.to_string())]);
        req_builder = req_builder.query(&[("with_runtime.gte", &with_runtime_gte.to_string())]);
        req_builder = req_builder.query(&[("with_runtime.lte", &with_runtime_lte.to_string())]);
        req_builder = req_builder.query(&[("include_null_first_air_dates", &include_null_first_air_dates.to_string())]);
        req_builder = req_builder.query(&[("with_original_language", &with_original_language.to_string())]);
        req_builder = req_builder.query(&[("without_keywords", &without_keywords.to_string())]);
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
