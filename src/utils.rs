use crate::VtResult;
use reqwest::{
    blocking::{multipart::Form, Client, Response},
    StatusCode,
};

use std::io::{BufRead, BufReader};

use serde::de::DeserializeOwned;

#[cfg(feature = "enterprise")]
use serde::Serialize;

#[inline]
fn process_resp<T>(resp: Response) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let status = resp.status();

    match status {
        StatusCode::OK => Ok(resp.json()?), // 200
        _ => Err((status, resp.text()?).into()),
    }
}

/// GET from a URL
pub(crate) fn http_get<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .send()?;
    process_resp(resp)
}

/// GET from a URL with query params
pub(crate) fn http_get_with_params<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    query_params: &[(&str, &str)],
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .query(query_params)
        .send()?;
    process_resp(resp)
}

/// POST to a URL
pub(crate) fn http_post<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: &[(&str, &str)],
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .form(form_data)
        .send()?;
    process_resp(resp)
}

/// POST to a URL with multipart form_data
pub(crate) fn http_multipart_post<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: Form,
) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .multipart(form_data)
        .send()?;
    process_resp(resp)
}

/// POST to a URL with data in the body
#[cfg(feature = "enterprise")]
pub(crate) fn http_body_post<S, T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    data: S,
) -> VtResult<T>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .json(&data)
        .send()?;
    process_resp(resp)
}

/// DELETE
pub(crate) fn http_delete<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<T>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .delete(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .send()?;
    process_resp(resp)
}

/// PATCH
#[cfg(feature = "enterprise")]
pub(crate) fn http_patch<S, T>(api_key: &str, user_agent: &str, url: &str, data: S) -> VtResult<T>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .patch(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .json(&data)
        .send()?;
    process_resp(resp)
}

#[inline]
fn process_resp_bz<T>(resp: Response) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let status = resp.status();

    match status {
        StatusCode::OK => {
            let read = bzip2::read::BzDecoder::new(resp);
            Ok(BufReader::new(read)
                .lines()
                .flatten()
                .filter_map(|line| serde_json::from_str(&line).ok())
                .collect()) // 200
        }
        _ => Err((status, resp.text()?).into()),
    }
}

/// GET from a URL
pub(crate) fn http_get_bz<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .send()?;
    process_resp_bz(resp)
}

/// GET from a URL with query params
pub(crate) fn http_get_with_params_bz<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    query_params: &[(&str, &str)],
) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .get(url)
        .header("x-apikey", api_key)
        .header("Accept", "application/json")
        .query(query_params)
        .send()?;
    process_resp_bz(resp)
}

/// POST to a URL
pub(crate) fn http_post_bz<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: &[(&str, &str)],
) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .form(form_data)
        .send()?;
    process_resp_bz(resp)
}

/// POST to a URL with multipart form_data
pub(crate) fn http_multipart_post_bz<T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    form_data: Form,
) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .multipart(form_data)
        .send()?;
    process_resp_bz(resp)
}

/// POST to a URL with data in the body
#[cfg(feature = "enterprise")]
pub(crate) fn http_body_post_bz<S, T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    data: S,
) -> VtResult<Vec<T>>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .post(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .json(&data)
        .send()?;
    process_resp_bz(resp)
}

/// DELETE
pub(crate) fn http_delete_bz<T>(api_key: &str, user_agent: &str, url: &str) -> VtResult<Vec<T>>
where
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .delete(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .send()?;
    process_resp_bz(resp)
}

/// PATCH
#[cfg(feature = "enterprise")]
pub(crate) fn http_patch_bz<S, T>(
    api_key: &str,
    user_agent: &str,
    url: &str,
    data: S,
) -> VtResult<Vec<T>>
where
    S: Serialize,
    T: DeserializeOwned,
{
    let client = Client::builder().user_agent(user_agent).build()?;
    let resp = client
        .patch(url)
        .header("x-apikey", api_key)
        // .header("Accept", "application/json")
        .json(&data)
        .send()?;
    process_resp_bz(resp)
}
