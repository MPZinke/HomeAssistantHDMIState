
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


#[derive(Debug, serde::Deserialize)]
struct Attributes
{}


#[derive(Debug, serde::Deserialize)]
struct Context
{
	id: String,
	parent_id: Option<String>,
	user_id: String,
}


#[derive(Debug, serde::Deserialize)]
pub struct InputBooleanEntity
{
    entity_id: String,
    pub state: String,
    attributes: Attributes,
    last_changed: String,
    last_reported: String,
    last_updated: String,
    context: Context,
}
