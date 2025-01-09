

use std::{thread, time};


use home_assistant::{get_tv_state, update_tv_state};
use hue::get_syncbox_state;


// FROM: https://stackoverflow.com/a/26390046
mod error;
mod home_assistant;
mod hue;
mod request;


fn main()
{
	let mut known_syncbox_state: Option<bool> = match get_tv_state()
	{
		Ok(tv_state) => Some(tv_state),
		Err(error) =>
		{
			println!("{}", error);
			None
		}
	};

	loop
	{
		match get_syncbox_state()
		{
			Err(error) => println!("{}", error),
			Ok(current_syncbox_state) =>
			{
				if known_syncbox_state.is_none_or(|known_syncbox_state| known_syncbox_state != current_syncbox_state)
				{
					match update_tv_state(current_syncbox_state)
					{
						Some(error) => println!("{}", error),
						None => known_syncbox_state = Some(current_syncbox_state),
					} 
				}
			},
		};

		static sleep_time: std::time::Duration = time::Duration::from_millis(2000);
		thread::sleep(sleep_time);
	}
}
