use std::env;
use std::process::Command;
use roblox_install::RobloxStudio;

fn main() -> Result<(), String> {
	let args: Vec<String> = env::args().collect();

	if args.len() != 2 {
		return Err(format!("Usage: {} <place.(rbxl|rbxlx)>", args[0]));
	}

	let roblox_studio = match RobloxStudio::locate() {
		Ok(object) => object,
		Err(err) => {
			return Err(format!("Failed to locate Roblox Studio: {}", err));
		}
	};
	
	let place_file_path = &args[1];
	
	if let Err(err) = Command::new(roblox_studio.application_path())
		.arg(place_file_path)
		.spawn() {
		return Err(format!("Failed to start Roblox Studio: {}", err));
	}

	Ok(())
}
