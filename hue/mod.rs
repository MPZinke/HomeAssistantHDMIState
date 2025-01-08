
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


#[allow(dead_code, non_snake_case)]
mod syncbox_execution;


use crate::request::*;
pub use crate::hue::syncbox_execution::SyncboxExecution;


static SYNCBOX_URL: &str = concat!("https://", env!("SYNCBOX_DOMAIN"), "/api/v1/execution");
static SYNCBOX_BEARER_AUTH: &str = concat!("Bearer ", env!("SYNCBOX_TOKEN"));


pub fn get_syncbox_execution() -> Result<SyncboxExecution, RequestError>
{
	let client: reqwest::blocking::Client = match get_request_client(&SYNCBOX_BEARER_AUTH)
	{
		Ok(client) => client,
		Err(error) => return Err(error),
	};

	let response = match client.get(SYNCBOX_URL).send()
	{
		Ok(response) => response,
		Err(error) => return Err(RequestError::ReqwestError(error)),
	};

	let response_body = match response.text()
	{
		Ok(response_body) => response_body,
		Err(error) => return Err(RequestError::ReqwestTextError(error)),
	};

	return match serde_json::from_str(&response_body)
	{
		Ok(syncbox_execution) => Ok(syncbox_execution),
		Err(error) => Err(RequestError::SerdeJsonError(error)),
	};
}
