
/***********************************************************************************************************************
*                                                                                                                      *
*   created by: MPZinke                                                                                                *
*   on 2025.01.08                                                                                                      *
*                                                                                                                      *
*   DESCRIPTION: TEMPLATE                                                                                              *
*   BUGS:                                                                                                              *
*   FUTURE:                                                                                                            *
*                                                                                                                      *
***********************************************************************************************************************/


use reqwest;


pub enum RequestError
{
	HTTPError(reqwest::Error),
	ReqwestClientError(reqwest::Error),
	ReqwestTextError(reqwest::Error),
	StatusError(std::io::Error),
}


impl std::fmt::Display for RequestError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			RequestError::HTTPError(error) => write!(format, "HTTPError: {}", error),
			RequestError::ReqwestClientError(error) => write!(format, "ReqwestClientError: {}", error),
			RequestError::ReqwestTextError(error) => write!(format, "ReqwestTextError: {}", error),
			RequestError::StatusError(error) => write!(format, "StatusError: {}", error),
		}
	}
}


pub enum JSONError
{
	DecodeError(serde_json::Error),
	KeyError(std::io::Error),
}


impl std::fmt::Display for JSONError
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			JSONError::DecodeError(error) => write!(format, "DecodeError: {}", error),
			JSONError::KeyError(error) => write!(format, "KeyError: {}", error),
		}
	}
}


pub enum Error
{
	JSONError(JSONError),
	RequestError(RequestError),
}


impl std::fmt::Display for Error
{
	fn fmt(&self, format: &mut std::fmt::Formatter) -> std::fmt::Result
	{
		match self
		{
			Error::JSONError(error) => write!(format, "{}", error),
			Error::RequestError(error) => write!(format, "{}", error),
		}
	}
}
