//code taken from https://github.com/nfarnan/cs1666_examples
use sdl2::rect::Rect;

pub struct SDLCore {
	pub sdl_cxt: sdl2::Sdl,
	pub wincan: sdl2::render::WindowCanvas,
	pub event_pump: sdl2::EventPump,
	pub cam: Rect,
}

impl SDLCore {
	pub fn init(
		title: &str,
		vsync: bool,
		width: u32,
		height: u32,
	) -> Result<SDLCore, String>
	{
		let sdl_cxt = sdl2::init()?;
		let video_subsys = sdl_cxt.video()?;

		let window = video_subsys.window(title, width, height)
			.build()
			.map_err(|e| e.to_string())?;

		let wincan = window.into_canvas().accelerated();

		// Check if we should lock to vsync
		let wincan = if vsync {
			wincan.present_vsync()
		}
		else {
			wincan
		};
		
		let wincan = wincan.build()
			.map_err(|e| e.to_string())?;

		let event_pump = sdl_cxt.event_pump()?;

		let cam = Rect::new(0, 0, width, height);

		Ok(SDLCore{
			sdl_cxt,
			wincan,
			event_pump,
			cam,
		})
	}
}