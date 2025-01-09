
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


use reqwest::blocking::Response;


use crate::error::RequestError;
use crate::error::RequestError::{HTTPError, ReqwestClientError};


fn build_request_client() -> Result<reqwest::blocking::Client, RequestError>
{
	return match reqwest::blocking::Client::builder()
	.danger_accept_invalid_certs(true)
	.build()
	{
		Ok(client) => Ok(client),
		Err(error) => Err(ReqwestClientError(error)),
	}
}


pub fn make_get_request(url: &str, auth_value: &str) -> Result<Response, RequestError>
{
	let client: reqwest::blocking::Client = match build_request_client()
	{
		Ok(client) => client,
		Err(error) => return Err(error),
	};

	return match client.get(url).bearer_auth(auth_value).send()
	{
		Ok(response) => Ok(response),
		Err(error) => return Err(HTTPError(error)),
	};
}


pub fn make_post_request(url: &str, auth_value: &str, body: &'static str) -> Result<Response, RequestError>
{
	let client: reqwest::blocking::Client = match build_request_client()
	{
		Ok(client) => client,
		Err(error) => return Err(error),
	};

	let response = match client
	.post(url)
	.bearer_auth(auth_value)
	.body(body)
	.send()
	{
		Ok(response) => response,
		Err(error) => return Err(HTTPError(error)),
	};

	return Ok(response)
}
