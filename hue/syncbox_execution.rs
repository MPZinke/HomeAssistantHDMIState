
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
struct Video
{
	intensity: String,
	backgroundLighting: bool,
}


#[derive(Debug, serde::Deserialize)]
struct Game
{
	intensity: String,
	backgroundLighting: bool,
}


#[derive(Debug, serde::Deserialize)]
struct Music
{
	intensity: String,
	palette: String,
}


#[derive(Debug, serde::Deserialize)]
pub struct SyncboxExecution
{
	mode: String,
	syncActive: bool,
	pub hdmiActive: bool,
	hdmiSource: String,
	hueTarget: String,
	brightness: i32,
	lastSyncMode: String,
	video: Video,
	game: Game,
	music: Music,
	preset: core::option::Option<String>,
}
