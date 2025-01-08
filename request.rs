
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2025.01.07                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use std::io::Error;


use serde_json;
use reqwest::header::AUTHORIZATION;
use reqwest::header::HeaderMap;
use reqwest::header::InvalidHeaderValue;


pub enum RequestError
{
	ReqwestError(reqwest::Error),
	ReqwestClientError(reqwest::Error),
	ReqwestTextError(reqwest::Error),
	InvalidHeaderValue(InvalidHeaderValue),
	SerdeJsonError(serde_json::Error),
	StatusError(Error),
}


impl std::fmt::Display for RequestError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			RequestError::ReqwestError(error) => write!(format, "reqwest::Error: {}", error),
			RequestError::ReqwestClientError(error) => write!(format, "reqwest::Error [Client]: {}", error),
			RequestError::ReqwestTextError(error) => write!(format, "reqwest::Error [Text]: {}", error),
			RequestError::InvalidHeaderValue(error) => write!(format, "reqwest::header::InvalidHeaderValue: {}", error),
			RequestError::SerdeJsonError(error) => write!(format, "serde_json::Error: {}", error),
			RequestError::StatusError(error) => write!(format, "StatusError: {}", error),
		}
	}
}


pub fn get_request_headers(auth_value: &str) -> Result<HeaderMap, InvalidHeaderValue>
{
	let mut headers = reqwest::header::HeaderMap::new();
	let auth_value = match reqwest::header::HeaderValue::from_str(auth_value)
	{
		Ok(auth_value) => auth_value,
		Err(err) => return Err(err),
	};

	headers.insert(AUTHORIZATION, auth_value);

	return Ok(headers);
}


pub fn get_request_client(auth_value: &str) -> Result<reqwest::blocking::Client, RequestError>
{
	let headers = match get_request_headers(auth_value)
	{
		Ok(headers) => headers,
		Err(error) => return Err(RequestError::InvalidHeaderValue(error)),
	};

	return match reqwest::blocking::Client::builder()
		.danger_accept_invalid_certs(true)
		.default_headers(headers)
		.build()
	{
		Ok(client) => Ok(client),
		Err(error) => Err(RequestError::ReqwestClientError(error)),
	}
}
