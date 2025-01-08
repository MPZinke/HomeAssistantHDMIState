

use std::{thread, time};


use home_assistant::{get_tv_state, update_tv_state};
use hue::get_syncbox_execution;


// FROM: https://stackoverflow.com/a/26390046
mod home_assistant;
mod hue;
mod request;


fn main()
{
	let mut syncbox_state: Option<bool> = match get_tv_state()
	{
		Ok(tv_state) => Some(tv_state.state == "on"),
		Err(error) =>
		{
			println!("{}", error);
			None
		}
	};

	loop
	{
		match get_syncbox_execution()
		{
			Err(error) => println!("{}", error),
			Ok(syncbox_execution)
			=>
			{
				// println!("syncbox_execution.hdmiActive: {}", syncbox_execution.hdmiActive);
				if syncbox_state.is_none_or(|syncbox_state| syncbox_state != syncbox_execution.hdmiActive)
				{
					match update_tv_state(syncbox_execution.hdmiActive)
					{
						Some(error) => println!("{}", error),
						None => syncbox_state = Some(syncbox_execution.hdmiActive),
					} 
				}
			},
		};

		let sleep_time: std::time::Duration = time::Duration::from_millis(2000);
		thread::sleep(sleep_time);
	}
}
