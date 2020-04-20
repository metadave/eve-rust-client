/* 
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * OpenAPI spec version: 1.3.8
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct CalendarApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> CalendarApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> CalendarApiClient<C> {
        CalendarApiClient {
            configuration: configuration,
        }
    }
}

pub trait CalendarApi {
    fn get_characters_character_id_calendar(&self, character_id: i32, datasource: &str, from_event: i32, if_none_match: &str, token: &str) -> Box<dyn Future<Item = Vec<::models::GetCharactersCharacterIdCalendar200Ok>, Error = Error<serde_json::Value>>>;
    fn get_characters_character_id_calendar_event_id(&self, character_id: i32, event_id: i32, datasource: &str, if_none_match: &str, token: &str) -> Box<dyn Future<Item = ::models::GetCharactersCharacterIdCalendarEventIdOk, Error = Error<serde_json::Value>>>;
    fn get_characters_character_id_calendar_event_id_attendees(&self, character_id: i32, event_id: i32, datasource: &str, if_none_match: &str, token: &str) -> Box<dyn Future<Item = Vec<::models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok>, Error = Error<serde_json::Value>>>;
    fn put_characters_character_id_calendar_event_id(&self, character_id: i32, event_id: i32, response: ::models::PutCharactersCharacterIdCalendarEventIdResponse, datasource: &str, token: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>CalendarApi for CalendarApiClient<C> {
    fn get_characters_character_id_calendar(&self, character_id: i32, datasource: &str, from_event: i32, if_none_match: &str, token: &str) -> Box<dyn Future<Item = Vec<::models::GetCharactersCharacterIdCalendar200Ok>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("from_event", &from_event.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/characters/{character_id}/calendar/?{}", configuration.base_path, query_string, character_id=character_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<Vec<::models::GetCharactersCharacterIdCalendar200Ok>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_characters_character_id_calendar_event_id(&self, character_id: i32, event_id: i32, datasource: &str, if_none_match: &str, token: &str) -> Box<dyn Future<Item = ::models::GetCharactersCharacterIdCalendarEventIdOk, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v3/characters/{character_id}/calendar/{event_id}/?{}", configuration.base_path, query_string, character_id=character_id, event_id=event_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<::models::GetCharactersCharacterIdCalendarEventIdOk, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn get_characters_character_id_calendar_event_id_attendees(&self, character_id: i32, event_id: i32, datasource: &str, if_none_match: &str, token: &str) -> Box<dyn Future<Item = Vec<::models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok>, Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v1/characters/{character_id}/calendar/{event_id}/attendees/?{}", configuration.base_path, query_string, character_id=character_id, event_id=event_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let headers = req.headers_mut();
            headers.set_raw("If-None-Match", if_none_match);
        }

        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }


        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|body| {
                let parsed: Result<Vec<::models::GetCharactersCharacterIdCalendarEventIdAttendees200Ok>, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            })
        )
    }

    fn put_characters_character_id_calendar_event_id(&self, character_id: i32, event_id: i32, response: ::models::PutCharactersCharacterIdCalendarEventIdResponse, datasource: &str, token: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let mut auth_headers = HashMap::<String, String>::new();
        let auth_query = HashMap::<String, String>::new();
        if let Some(ref token) = configuration.oauth_access_token {
            let auth = hyper::header::Authorization(
                hyper::header::Bearer {
                    token: token.to_owned(),
                }
            );
            auth_headers.insert("Authorization".to_owned(), auth.to_string());
        };
        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.append_pair("datasource", &datasource.to_string());
            query.append_pair("token", &token.to_string());
            for (key, val) in &auth_query {
                query.append_pair(key, val);
            }
            query.finish()
        };
        let uri_str = format!("{}/v3/characters/{character_id}/calendar/{event_id}/?{}", configuration.base_path, query_string, character_id=character_id, event_id=event_id);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }


        for (key, val) in auth_headers {
            req.headers_mut().set_raw(key, val);
        }

        let serialized = serde_json::to_string(&response).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

}