
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


use std::io::ErrorKind;


use serde_json;


use crate::error::Error;
use crate::error::Error::{JSONError, RequestError};
use crate::error::JSONError::{DecodeError, KeyError};
use crate::error::RequestError::ReqwestTextError;
use crate::request::make_get_request;


static SYNCBOX_URL: &str = concat!("https://", env!("SYNCBOX_DOMAIN"), "/api/v1/execution");
static SYNCBOX_BEARER_AUTH: &str = env!("SYNCBOX_TOKEN");


pub fn get_syncbox_state() -> Result<bool, Error>
{
	let response = match make_get_request(SYNCBOX_URL, SYNCBOX_BEARER_AUTH)
	{
		Ok(response) => response,
		Err(error) => return Err(RequestError(error)),
	};

	let response_body = match response.text()
	{
		Ok(response_body) => response_body,
		Err(error) => return Err(RequestError(ReqwestTextError(error))),
	};

	let syncbox_execution: serde_json::Value = match serde_json::from_str::<serde_json::Value>(&response_body)
	{
		Ok(syncbox_execution) => syncbox_execution,
		Err(error) => return Err(JSONError(DecodeError(error))),
	};

	return match syncbox_execution["hdmiActive"]
	{
		serde_json::Value::Bool(hdmi_active) => Ok(hdmi_active),
		_ => Err(JSONError(KeyError(std::io::Error::new(ErrorKind::Other, "Bad JSON format.")))),
	};
}
