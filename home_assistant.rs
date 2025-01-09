
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
use crate::error::RequestError::{ReqwestTextError, StatusError};
use crate::request::{make_get_request, make_post_request};


static HOME_ASSISTANT_URL: &str = "http://homeassistant.local:8123/api/states/input_boolean.tv_state";
static HOME_ASSISTANT_AUTH: &str = env!("HOME_ASSISTANT_TOKEN");


static STATE_ON: &str = r#"{"state": "on"}"#;
static STATE_OFF: &str = r#"{"state": "off"}"#;


pub fn get_tv_state() -> Result<bool, Error>
{
	let response = match make_get_request(&HOME_ASSISTANT_URL, &HOME_ASSISTANT_AUTH)
	{
		Ok(response) => response,
		Err(error) => return Err(RequestError(error)),
	};

	let response_body = match response.text()
	{
		Ok(response_body) => response_body,
		Err(error) => return Err(RequestError(ReqwestTextError(error))),
	};

	let tv_state_entity: serde_json::Value = match serde_json::from_str::<serde_json::Value>(&response_body)
	{
		Ok(tv_state_entity) => tv_state_entity,
		Err(error) => return Err(JSONError(DecodeError(error))),
	};

	return match &tv_state_entity["state"]
	{
		serde_json::Value::String(tv_state_state) => Ok(tv_state_state == "on"),
		_ => Err(JSONError(KeyError(std::io::Error::new(ErrorKind::Other, "Bad JSON format.")))),
	};
}


pub fn update_tv_state(hdmi_is_active: bool) -> Option<Error>
{
	let body: &str;
	if hdmi_is_active
	{
		body = &STATE_ON;
		println!(r#"Setting TV state "on""#);
	}
	else
	{
		body = &STATE_OFF;
		println!(r#"Setting TV state "off""#);
	}

	let response = match make_post_request(&HOME_ASSISTANT_URL, &HOME_ASSISTANT_AUTH, &body)
	{
		Ok(response) => response,
		Err(error) => return Some(RequestError(error)),
	};

	if response.status().is_success()
	{
		return None;
	}

	return Some(RequestError(StatusError(std::io::Error::new(ErrorKind::Other, format!("Status Code: {}", response.status())))));
}
