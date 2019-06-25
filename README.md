# Rust API client for openapi

## Welcome  This is a place to put general notes and extra information, for internal use.  To get started designing/documenting this API, select a version on the left. # Title No Description

## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 3
- Package version: 1.0.0
- Build package: org.openapitools.codegen.languages.RustClientCodegen

## Installation

Put the package under your project folder and add the following to `Cargo.toml` under `[dependencies]`:

```
    openapi = { path = "./generated" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.themoviedb.org/3*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**get_account_favorite_movies_paginated**](docs/AccountApi.md#get_account_favorite_movies_paginated) | **get** /account/{account_id}/favorite/movies | Get Favorite Movies
*AccountApi* | [**get_account_favorite_tv_paginated**](docs/AccountApi.md#get_account_favorite_tv_paginated) | **get** /account/{account_id}/favorite/tv | Get Favorite TV Shows
*AccountApi* | [**get_account_rated_movies_paginated**](docs/AccountApi.md#get_account_rated_movies_paginated) | **get** /account/{account_id}/rated/movies | Get Rated Movies
*AccountApi* | [**get_account_rated_tv_episodes_paginated**](docs/AccountApi.md#get_account_rated_tv_episodes_paginated) | **get** /account/{account_id}/rated/tv/episodes | Get Rated TV Episodes
*AccountApi* | [**get_account_rated_tv_paginated**](docs/AccountApi.md#get_account_rated_tv_paginated) | **get** /account/{account_id}/rated/tv | Get Rated TV Shows
*AccountApi* | [**get_account_watchlist_movies_paginated**](docs/AccountApi.md#get_account_watchlist_movies_paginated) | **get** /account/{account_id}/watchlist/movies | Get Movie Watchlist
*AccountApi* | [**get_account_watchlist_tv_paginated**](docs/AccountApi.md#get_account_watchlist_tv_paginated) | **get** /account/{account_id}/watchlist/tv | Get TV Show Watchlist
*AccountApi* | [**get_current_account_details**](docs/AccountApi.md#get_current_account_details) | **get** /account | Get Details
*AccountApi* | [**get_current_account_lists_paginated**](docs/AccountApi.md#get_current_account_lists_paginated) | **get** /account/{account_id}/lists | Get Created Lists
*AccountApi* | [**post_account_favorite**](docs/AccountApi.md#post_account_favorite) | **post** /account/{account_id}/favorite | Mark as Favorite
*AccountApi* | [**post_account_watchlist**](docs/AccountApi.md#post_account_watchlist) | **post** /account/{account_id}/watchlist | Add to Watchlist
*AuthenticationApi* | [**get_new_authentication_guest_session**](docs/AuthenticationApi.md#get_new_authentication_guest_session) | **get** /authentication/guest_session/new | Create Guest Session
*AuthenticationApi* | [**get_new_authentication_session**](docs/AuthenticationApi.md#get_new_authentication_session) | **get** /authentication/session/new | Create Session
*AuthenticationApi* | [**get_new_authentication_token**](docs/AuthenticationApi.md#get_new_authentication_token) | **get** /authentication/token/new | Create Request Token
*AuthenticationApi* | [**get_validate_authentication_token_with_login**](docs/AuthenticationApi.md#get_validate_authentication_token_with_login) | **get** /authentication/token/validate_with_login | Validate Request Token
*CertificationsApi* | [**get_movie_certifications_list**](docs/CertificationsApi.md#get_movie_certifications_list) | **get** /certification/movie/list | Get Movie Certifications
*CertificationsApi* | [**get_tv_certifications_list**](docs/CertificationsApi.md#get_tv_certifications_list) | **get** /certification/tv/list | Get TV Certifications
*ChangesApi* | [**get_movie_changes_paginated**](docs/ChangesApi.md#get_movie_changes_paginated) | **get** /movie/changes | Get Movie Change List
*ChangesApi* | [**get_person_changes_paginated**](docs/ChangesApi.md#get_person_changes_paginated) | **get** /person/changes | Get Person Change List
*ChangesApi* | [**get_tv_changes_paginated**](docs/ChangesApi.md#get_tv_changes_paginated) | **get** /tv/changes | Get TV Change List
*CollectionsApi* | [**get_collection_details**](docs/CollectionsApi.md#get_collection_details) | **get** /collection/{collection_id} | Get Details
*CollectionsApi* | [**get_collection_images_list**](docs/CollectionsApi.md#get_collection_images_list) | **get** /collection/{collection_id}/images | Get Images
*CompaniesApi* | [**get_company_details**](docs/CompaniesApi.md#get_company_details) | **get** /company/{company_id} | Get Details
*CompaniesApi* | [**get_company_movies_paginated**](docs/CompaniesApi.md#get_company_movies_paginated) | **get** /company/{company_id}/movies | Get Movies
*ConfigurationApi* | [**get_configuration**](docs/ConfigurationApi.md#get_configuration) | **get** /configuration | Get API Configuration
*CreditsApi* | [**get_credit_details**](docs/CreditsApi.md#get_credit_details) | **get** /credit/{credit_id} | Get Details
*DiscoverApi* | [**get_discover_movie_paginated**](docs/DiscoverApi.md#get_discover_movie_paginated) | **get** /discover/movie | Movie Discover
*DiscoverApi* | [**get_discover_tv_paginated**](docs/DiscoverApi.md#get_discover_tv_paginated) | **get** /discover/tv | TV Discover
*FindApi* | [**get_find_external_id**](docs/FindApi.md#get_find_external_id) | **get** /find/{external_id} | Find by ID
*GenresApi* | [**get_all_movie_genres_list**](docs/GenresApi.md#get_all_movie_genres_list) | **get** /genre/movie/list | Get Movie List
*GenresApi* | [**get_all_tv_genres_list**](docs/GenresApi.md#get_all_tv_genres_list) | **get** /genre/tv/list | Get TV List
*GenresApi* | [**get_movies_by_genre_paginated**](docs/GenresApi.md#get_movies_by_genre_paginated) | **get** /genre/{genre_id}/movies | Get Movies
*GuestSessionsApi* | [**get_guest_session_rated_movies_paginated**](docs/GuestSessionsApi.md#get_guest_session_rated_movies_paginated) | **get** /guest_session/{guest_session_id}/rated/movies | Get Rated Movies
*GuestSessionsApi* | [**get_guest_session_rated_tv_episodes_paginated**](docs/GuestSessionsApi.md#get_guest_session_rated_tv_episodes_paginated) | **get** /guest_session/{guest_session_id}/rated/tv/episodes | Get Rated TV Episodes
*GuestSessionsApi* | [**get_guest_session_rated_tv_paginated**](docs/GuestSessionsApi.md#get_guest_session_rated_tv_paginated) | **get** /guest_session/{guest_session_id}/rated/tv | Get Rated TV Shows
*JobsApi* | [**get_jobs_list**](docs/JobsApi.md#get_jobs_list) | **get** /job/list | Get Jobs
*KeywordsApi* | [**get_keyword_details**](docs/KeywordsApi.md#get_keyword_details) | **get** /keyword/{keyword_id} | Get Details
*KeywordsApi* | [**get_movies_by_keyword_paginated**](docs/KeywordsApi.md#get_movies_by_keyword_paginated) | **get** /keyword/{keyword_id}/movies | Get Movies
*ListsApi* | [**get_list_details**](docs/ListsApi.md#get_list_details) | **get** /list/{list_id} | Get Details
*ListsApi* | [**get_list_item_status**](docs/ListsApi.md#get_list_item_status) | **get** /list/{list_id}/item_status | Check Item Status
*ListsApi* | [**post_list**](docs/ListsApi.md#post_list) | **post** /list | Create List
*ListsApi* | [**post_list_add_item**](docs/ListsApi.md#post_list_add_item) | **post** /list/{list_id}/add_item | Add Movie
*ListsApi* | [**post_list_clear**](docs/ListsApi.md#post_list_clear) | **post** /list/{list_id}/clear | Clear List
*ListsApi* | [**post_list_remove_item**](docs/ListsApi.md#post_list_remove_item) | **post** /list/{list_id}/remove_item | Remove Movie
*MoviesApi* | [**delete_movie_rating**](docs/MoviesApi.md#delete_movie_rating) | **delete** /movie/{movie_id}/rating | Delete Rating
*MoviesApi* | [**get_movie_account_states**](docs/MoviesApi.md#get_movie_account_states) | **get** /movie/{movie_id}/account_states | Get Account States
*MoviesApi* | [**get_movie_alternative_titles_list**](docs/MoviesApi.md#get_movie_alternative_titles_list) | **get** /movie/{movie_id}/alternative_titles | Get Alternative Titles
*MoviesApi* | [**get_movie_changes_list**](docs/MoviesApi.md#get_movie_changes_list) | **get** /movie/{movie_id}/changes | Get Changes
*MoviesApi* | [**get_movie_credits**](docs/MoviesApi.md#get_movie_credits) | **get** /movie/{movie_id}/credits | Get Credits
*MoviesApi* | [**get_movie_details**](docs/MoviesApi.md#get_movie_details) | **get** /movie/{movie_id} | Get Details
*MoviesApi* | [**get_movie_images**](docs/MoviesApi.md#get_movie_images) | **get** /movie/{movie_id}/images | Get Images
*MoviesApi* | [**get_movie_keywords_list**](docs/MoviesApi.md#get_movie_keywords_list) | **get** /movie/{movie_id}/keywords | Get Keywords
*MoviesApi* | [**get_movie_latest_details**](docs/MoviesApi.md#get_movie_latest_details) | **get** /movie/latest | Get Latest
*MoviesApi* | [**get_movie_lists_paginated**](docs/MoviesApi.md#get_movie_lists_paginated) | **get** /movie/{movie_id}/lists | Get Lists
*MoviesApi* | [**get_movie_now_playing_paginated**](docs/MoviesApi.md#get_movie_now_playing_paginated) | **get** /movie/now_playing | Get Now Playing
*MoviesApi* | [**get_movie_popular_paginated**](docs/MoviesApi.md#get_movie_popular_paginated) | **get** /movie/popular | Get Popular
*MoviesApi* | [**get_movie_recommendations_paginated**](docs/MoviesApi.md#get_movie_recommendations_paginated) | **get** /movie/{movie_id}/recommendations | Get Recommendations
*MoviesApi* | [**get_movie_release_dates**](docs/MoviesApi.md#get_movie_release_dates) | **get** /movie/{movie_id}/release_dates | Get Release Dates
*MoviesApi* | [**get_movie_reviews_paginated**](docs/MoviesApi.md#get_movie_reviews_paginated) | **get** /movie/{movie_id}/reviews | Get Reviews
*MoviesApi* | [**get_movie_similar_paginated**](docs/MoviesApi.md#get_movie_similar_paginated) | **get** /movie/{movie_id}/similar | Get Similar Movies
*MoviesApi* | [**get_movie_top_rated_paginated**](docs/MoviesApi.md#get_movie_top_rated_paginated) | **get** /movie/top_rated | Get Top Rated
*MoviesApi* | [**get_movie_translations_list**](docs/MoviesApi.md#get_movie_translations_list) | **get** /movie/{movie_id}/translations | Get Translations
*MoviesApi* | [**get_movie_upcoming_paginated**](docs/MoviesApi.md#get_movie_upcoming_paginated) | **get** /movie/upcoming | Get Upcoming
*MoviesApi* | [**get_movie_videos_list**](docs/MoviesApi.md#get_movie_videos_list) | **get** /movie/{movie_id}/videos | Get Videos
*MoviesApi* | [**post_movie_rating**](docs/MoviesApi.md#post_movie_rating) | **post** /movie/{movie_id}/rating | Rate Movie
*NetworksApi* | [**get_network_details**](docs/NetworksApi.md#get_network_details) | **get** /network/{network_id} | Get Details
*PeopleApi* | [**get_person_changes**](docs/PeopleApi.md#get_person_changes) | **get** /person/{person_id}/changes | Get Changes
*PeopleApi* | [**get_person_combined_credits**](docs/PeopleApi.md#get_person_combined_credits) | **get** /person/{person_id}/combined_credits | Get Combined Credits
*PeopleApi* | [**get_person_details**](docs/PeopleApi.md#get_person_details) | **get** /person/{person_id} | Get Details
*PeopleApi* | [**get_person_external_ids**](docs/PeopleApi.md#get_person_external_ids) | **get** /person/{person_id}/external_ids | Get External IDs
*PeopleApi* | [**get_person_images_list**](docs/PeopleApi.md#get_person_images_list) | **get** /person/{person_id}/images | Get Images
*PeopleApi* | [**get_person_latest_details**](docs/PeopleApi.md#get_person_latest_details) | **get** /person/latest | Get Latest
*PeopleApi* | [**get_person_movie_credits**](docs/PeopleApi.md#get_person_movie_credits) | **get** /person/{person_id}/movie_credits | Get Movie Credits
*PeopleApi* | [**get_person_popular_paginated**](docs/PeopleApi.md#get_person_popular_paginated) | **get** /person/popular | Get Popular
*PeopleApi* | [**get_person_tagged_images_paginated**](docs/PeopleApi.md#get_person_tagged_images_paginated) | **get** /person/{person_id}/tagged_images | Get Tagged Images
*PeopleApi* | [**get_person_tv_credits**](docs/PeopleApi.md#get_person_tv_credits) | **get** /person/{person_id}/tv_credits | Get TV Credits
*ReviewsApi* | [**get_review_details**](docs/ReviewsApi.md#get_review_details) | **get** /review/{review_id} | Get Details
*SearchApi* | [**get_search_collection_paginated**](docs/SearchApi.md#get_search_collection_paginated) | **get** /search/collection | Search Collections
*SearchApi* | [**get_search_company_paginated**](docs/SearchApi.md#get_search_company_paginated) | **get** /search/company | Search Companies
*SearchApi* | [**get_search_keyword_paginated**](docs/SearchApi.md#get_search_keyword_paginated) | **get** /search/keyword | Search Keywords
*SearchApi* | [**get_search_movie_paginated**](docs/SearchApi.md#get_search_movie_paginated) | **get** /search/movie | Search Movies
*SearchApi* | [**get_search_multi_paginated**](docs/SearchApi.md#get_search_multi_paginated) | **get** /search/multi | Multi Search
*SearchApi* | [**get_search_person_paginated**](docs/SearchApi.md#get_search_person_paginated) | **get** /search/person | Search People
*SearchApi* | [**get_search_tv_paginated**](docs/SearchApi.md#get_search_tv_paginated) | **get** /search/tv | Search TV Shows
*TVApi* | [**delete_tv_rating**](docs/TVApi.md#delete_tv_rating) | **delete** /tv/{tv_id}/rating | Delete Rating
*TVApi* | [**get_tv_account_states**](docs/TVApi.md#get_tv_account_states) | **get** /tv/{tv_id}/account_states | Get Account States
*TVApi* | [**get_tv_airing_today_paginated**](docs/TVApi.md#get_tv_airing_today_paginated) | **get** /tv/airing_today | Get TV Airing Today
*TVApi* | [**get_tv_alternative_titles_list**](docs/TVApi.md#get_tv_alternative_titles_list) | **get** /tv/{tv_id}/alternative_titles | Get Alternative Titles
*TVApi* | [**get_tv_changes**](docs/TVApi.md#get_tv_changes) | **get** /tv/{tv_id}/changes | Get Changes
*TVApi* | [**get_tv_content_ratings_list**](docs/TVApi.md#get_tv_content_ratings_list) | **get** /tv/{tv_id}/content_ratings | Get Content Ratings
*TVApi* | [**get_tv_credits**](docs/TVApi.md#get_tv_credits) | **get** /tv/{tv_id}/credits | Get Credits
*TVApi* | [**get_tv_details**](docs/TVApi.md#get_tv_details) | **get** /tv/{tv_id} | Get Details
*TVApi* | [**get_tv_external_ids**](docs/TVApi.md#get_tv_external_ids) | **get** /tv/{tv_id}/external_ids | Get External IDs
*TVApi* | [**get_tv_images**](docs/TVApi.md#get_tv_images) | **get** /tv/{tv_id}/images | Get Images
*TVApi* | [**get_tv_keywords_list**](docs/TVApi.md#get_tv_keywords_list) | **get** /tv/{tv_id}/keywords | Get Keywords
*TVApi* | [**get_tv_latest_details**](docs/TVApi.md#get_tv_latest_details) | **get** /tv/latest | Get Latest
*TVApi* | [**get_tv_on_the_air_paginated**](docs/TVApi.md#get_tv_on_the_air_paginated) | **get** /tv/on_the_air | Get TV On The Air
*TVApi* | [**get_tv_popular_paginated**](docs/TVApi.md#get_tv_popular_paginated) | **get** /tv/popular | Get Popular
*TVApi* | [**get_tv_recommendations_paginated**](docs/TVApi.md#get_tv_recommendations_paginated) | **get** /tv/{tv_id}/recommendations | Get Recommendations
*TVApi* | [**get_tv_similar_paginated**](docs/TVApi.md#get_tv_similar_paginated) | **get** /tv/{tv_id}/similar | Get Similar TV Shows
*TVApi* | [**get_tv_top_rated_paginated**](docs/TVApi.md#get_tv_top_rated_paginated) | **get** /tv/top_rated | Get Top Rated
*TVApi* | [**get_tv_translations_list**](docs/TVApi.md#get_tv_translations_list) | **get** /tv/{tv_id}/translations | Get Translations
*TVApi* | [**get_tv_videos_list**](docs/TVApi.md#get_tv_videos_list) | **get** /tv/{tv_id}/videos | Get Videos
*TVApi* | [**post_tv_rating**](docs/TVApi.md#post_tv_rating) | **post** /tv/{tv_id}/rating | Rate TV Show
*TVEpisodesApi* | [**delete_tv_season_episode_rating**](docs/TVEpisodesApi.md#delete_tv_season_episode_rating) | **delete** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Delete Rating
*TVEpisodesApi* | [**get_tv_episode_changes**](docs/TVEpisodesApi.md#get_tv_episode_changes) | **get** /tv/episode/{episode_id}/changes | Get Changes
*TVEpisodesApi* | [**get_tv_season_episode_account_states**](docs/TVEpisodesApi.md#get_tv_season_episode_account_states) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/account_states | Get Account States
*TVEpisodesApi* | [**get_tv_season_episode_credits**](docs/TVEpisodesApi.md#get_tv_season_episode_credits) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/credits | Get Credits
*TVEpisodesApi* | [**get_tv_season_episode_details**](docs/TVEpisodesApi.md#get_tv_season_episode_details) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number} | Get Details
*TVEpisodesApi* | [**get_tv_season_episode_external_ids**](docs/TVEpisodesApi.md#get_tv_season_episode_external_ids) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/external_ids | Get TV Episode External IDs
*TVEpisodesApi* | [**get_tv_season_episode_images**](docs/TVEpisodesApi.md#get_tv_season_episode_images) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/images | Get Images
*TVEpisodesApi* | [**get_tv_season_episode_videos_list**](docs/TVEpisodesApi.md#get_tv_season_episode_videos_list) | **get** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/videos | Get  Videos
*TVEpisodesApi* | [**post_tv_season_episode_rating**](docs/TVEpisodesApi.md#post_tv_season_episode_rating) | **post** /tv/{tv_id}/season/{season_number}/episode/{episode_number}/rating | Rate TV Episode
*TVSeasonsApi* | [**get_tv_season_account_states**](docs/TVSeasonsApi.md#get_tv_season_account_states) | **get** /tv/{tv_id}/season/{season_number}/account_states | Get Account States
*TVSeasonsApi* | [**get_tv_season_changes**](docs/TVSeasonsApi.md#get_tv_season_changes) | **get** /tv/season/{season_id}/changes | Get  Changes
*TVSeasonsApi* | [**get_tv_season_credits**](docs/TVSeasonsApi.md#get_tv_season_credits) | **get** /tv/{tv_id}/season/{season_number}/credits | Get Credits
*TVSeasonsApi* | [**get_tv_season_details**](docs/TVSeasonsApi.md#get_tv_season_details) | **get** /tv/{tv_id}/season/{season_number} | Get Details
*TVSeasonsApi* | [**get_tv_season_external_ids**](docs/TVSeasonsApi.md#get_tv_season_external_ids) | **get** /tv/{tv_id}/season/{season_number}/external_ids | Get External IDs
*TVSeasonsApi* | [**get_tv_season_images**](docs/TVSeasonsApi.md#get_tv_season_images) | **get** /tv/{tv_id}/season/{season_number}/images | Get Images
*TVSeasonsApi* | [**get_tv_season_videos**](docs/TVSeasonsApi.md#get_tv_season_videos) | **get** /tv/{tv_id}/season/{season_number}/videos | Get Videos
*TimezonesApi* | [**get_timezones_list**](docs/TimezonesApi.md#get_timezones_list) | **get** /timezones/list | Get List


## Documentation For Models

 - [AccountDetails](docs/AccountDetails.md)
 - [AccountStates](docs/AccountStates.md)
 - [AccountdetailsAvatar](docs/AccountdetailsAvatar.md)
 - [AccountdetailsAvatarGravatar](docs/AccountdetailsAvatarGravatar.md)
 - [AlternativeTitlesList](docs/AlternativeTitlesList.md)
 - [AlternativetitleslistTitles](docs/AlternativetitleslistTitles.md)
 - [Cast](docs/Cast.md)
 - [CastDetails](docs/CastDetails.md)
 - [Certification](docs/Certification.md)
 - [Certifications](docs/Certifications.md)
 - [CertificationsCertifications](docs/CertificationsCertifications.md)
 - [ChangeDetails](docs/ChangeDetails.md)
 - [ChangedetailsChanges](docs/ChangedetailsChanges.md)
 - [ChangedetailsItems](docs/ChangedetailsItems.md)
 - [ChangesObject](docs/ChangesObject.md)
 - [ChangesPaginated](docs/ChangesPaginated.md)
 - [ChangespaginatedAllOf](docs/ChangespaginatedAllOf.md)
 - [CollectionObject](docs/CollectionObject.md)
 - [CollectionPart](docs/CollectionPart.md)
 - [Company](docs/Company.md)
 - [CompanyDetails](docs/CompanyDetails.md)
 - [Configuration](docs/Configuration.md)
 - [ConfigurationImages](docs/ConfigurationImages.md)
 - [Creator](docs/Creator.md)
 - [Credit](docs/Credit.md)
 - [CreditMedia](docs/CreditMedia.md)
 - [CreditMediaSeasons](docs/CreditMediaSeasons.md)
 - [CreditPerson](docs/CreditPerson.md)
 - [Credits](docs/Credits.md)
 - [Crew](docs/Crew.md)
 - [CrewDetails](docs/CrewDetails.md)
 - [EpisodeDetails](docs/EpisodeDetails.md)
 - [EpisodeRatingList](docs/EpisodeRatingList.md)
 - [EpisoderatinglistRated](docs/EpisoderatinglistRated.md)
 - [EpisoderatinglistResults](docs/EpisoderatinglistResults.md)
 - [FindByExternalIdResults](docs/FindByExternalIdResults.md)
 - [Genre](docs/Genre.md)
 - [GenresList](docs/GenresList.md)
 - [GuestSessionResponse](docs/GuestSessionResponse.md)
 - [GuestStar](docs/GuestStar.md)
 - [Image](docs/Image.md)
 - [Images](docs/Images.md)
 - [InlineObject](docs/InlineObject.md)
 - [InlineObject1](docs/InlineObject1.md)
 - [InlineObject2](docs/InlineObject2.md)
 - [InlineObject3](docs/InlineObject3.md)
 - [InlineObject4](docs/InlineObject4.md)
 - [InlineObject5](docs/InlineObject5.md)
 - [InlineObject6](docs/InlineObject6.md)
 - [InlineObject7](docs/InlineObject7.md)
 - [InlineResponse401](docs/InlineResponse401.md)
 - [ItemStatus](docs/ItemStatus.md)
 - [Jobs](docs/Jobs.md)
 - [JobsJobs](docs/JobsJobs.md)
 - [Keyword](docs/Keyword.md)
 - [KeywordsList](docs/KeywordsList.md)
 - [ListDetails](docs/ListDetails.md)
 - [ListObject](docs/ListObject.md)
 - [ListStatusResponse](docs/ListStatusResponse.md)
 - [ListsPaginated](docs/ListsPaginated.md)
 - [ListspaginatedAllOf](docs/ListspaginatedAllOf.md)
 - [MediaType](docs/MediaType.md)
 - [MovieDetails](docs/MovieDetails.md)
 - [MovieObject](docs/MovieObject.md)
 - [MoviePaginated](docs/MoviePaginated.md)
 - [MovieTvExternalIds](docs/MovieTvExternalIds.md)
 - [MoviedetailsProductionCountries](docs/MoviedetailsProductionCountries.md)
 - [MoviedetailsSpokenLanguages](docs/MoviedetailsSpokenLanguages.md)
 - [MovieobjectDates](docs/MovieobjectDates.md)
 - [MoviepaginatedAllOf](docs/MoviepaginatedAllOf.md)
 - [Network](docs/Network.md)
 - [PersonCredits](docs/PersonCredits.md)
 - [PersonDetails](docs/PersonDetails.md)
 - [PersonExternalIds](docs/PersonExternalIds.md)
 - [PersonImagesList](docs/PersonImagesList.md)
 - [PersonObject](docs/PersonObject.md)
 - [PersonPopularPaginated](docs/PersonPopularPaginated.md)
 - [PersonTaggedImagesPaginated](docs/PersonTaggedImagesPaginated.md)
 - [PersonpopularpaginatedAllOf](docs/PersonpopularpaginatedAllOf.md)
 - [PersontaggedimagespaginatedAllOf](docs/PersontaggedimagespaginatedAllOf.md)
 - [RatingsList](docs/RatingsList.md)
 - [RatingslistResults](docs/RatingslistResults.md)
 - [ReleaseDate](docs/ReleaseDate.md)
 - [ReleaseDatesList](docs/ReleaseDatesList.md)
 - [ReleasedateslistResults](docs/ReleasedateslistResults.md)
 - [Review](docs/Review.md)
 - [ReviewObject](docs/ReviewObject.md)
 - [ReviewsPaginated](docs/ReviewsPaginated.md)
 - [ReviewspaginatedAllOf](docs/ReviewspaginatedAllOf.md)
 - [SearchCollectionResultsPaginated](docs/SearchCollectionResultsPaginated.md)
 - [SearchCompanyResultsPaginated](docs/SearchCompanyResultsPaginated.md)
 - [SearchKeywordResultsPaginated](docs/SearchKeywordResultsPaginated.md)
 - [SearchMultiResultsPaginated](docs/SearchMultiResultsPaginated.md)
 - [SearchPersonResultsPaginated](docs/SearchPersonResultsPaginated.md)
 - [SearchcollectionresultspaginatedAllOf](docs/SearchcollectionresultspaginatedAllOf.md)
 - [SearchcollectionresultspaginatedAllOfResults](docs/SearchcollectionresultspaginatedAllOfResults.md)
 - [SearchcompanyresultspaginatedAllOf](docs/SearchcompanyresultspaginatedAllOf.md)
 - [SearchcompanyresultspaginatedAllOfResults](docs/SearchcompanyresultspaginatedAllOfResults.md)
 - [SearchkeywordresultspaginatedAllOf](docs/SearchkeywordresultspaginatedAllOf.md)
 - [SearchmultiresultspaginatedAllOf](docs/SearchmultiresultspaginatedAllOf.md)
 - [SearchpersonresultspaginatedAllOf](docs/SearchpersonresultspaginatedAllOf.md)
 - [SearchpersonresultspaginatedAllOfResults](docs/SearchpersonresultspaginatedAllOfResults.md)
 - [SeasonDetails](docs/SeasonDetails.md)
 - [SeasonObject](docs/SeasonObject.md)
 - [SessionResponse](docs/SessionResponse.md)
 - [TaggedImage](docs/TaggedImage.md)
 - [TaggedimageAllOf](docs/TaggedimageAllOf.md)
 - [TokenResponse](docs/TokenResponse.md)
 - [TokenResponseWithExpiration](docs/TokenResponseWithExpiration.md)
 - [Traitid](docs/Traitid.md)
 - [Traitpaginated](docs/Traitpaginated.md)
 - [Translations](docs/Translations.md)
 - [TranslationsTranslations](docs/TranslationsTranslations.md)
 - [TvDetails](docs/TvDetails.md)
 - [TvEpisodeObject](docs/TvEpisodeObject.md)
 - [TvEpisodesPaginated](docs/TvEpisodesPaginated.md)
 - [TvObject](docs/TvObject.md)
 - [TvPaginated](docs/TvPaginated.md)
 - [TvepisodespaginatedAllOf](docs/TvepisodespaginatedAllOf.md)
 - [TvpaginatedAllOf](docs/TvpaginatedAllOf.md)
 - [Video](docs/Video.md)
 - [VideosList](docs/VideosList.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



