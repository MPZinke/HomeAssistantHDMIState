
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
mod entity;


use std::io::{Error, ErrorKind};


use crate::request::*;
pub use crate::home_assistant::entity::InputBooleanEntity;


static HOME_ASSISTANT_URL: &str = "http://homeassistant.local:8123/api/states/input_boolean.tv_state";
static HOME_ASSISTANT_AUTH: &str = concat!("Bearer ", env!("HOME_ASSISTANT_TOKEN"));


pub fn get_tv_state() -> Result<InputBooleanEntity, RequestError>
{
	let client: reqwest::blocking::Client = match get_request_client(&HOME_ASSISTANT_AUTH)
	{
		Ok(client) => client,
		Err(error) => return Err(error),
	};

	let response = match client.get(HOME_ASSISTANT_URL).send()
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
		Ok(tv_state) => Ok(tv_state),
		Err(error) => Err(RequestError::SerdeJsonError(error)),
	};
}


pub fn update_tv_state(hdmi_is_active: bool) -> Option<RequestError>
{
	let client: reqwest::blocking::Client = match get_request_client(&HOME_ASSISTANT_AUTH)
	{
		Ok(client) => client,
		Err(error) => return Some(error),
	};

	let body: &str;
	if hdmi_is_active
	{
		body = r#"{"state": "on"}"#;
		println!("HDMI is active");
	}
	else
	{
		body = r#"{"state": "off"}"#;
		println!("HDMI is inactive");
	}

	let response = match client.post(HOME_ASSISTANT_URL)
	.body(body)
	.send()
	{
		Ok(response) => response,
		Err(error) => return Some(RequestError::ReqwestError(error)),
	};

	if response.status().is_success()
	{
		return None;
	}

	return Some(RequestError::StatusError(Error::new(ErrorKind::Other, format!("Status Code: {}", response.status()))));
}
