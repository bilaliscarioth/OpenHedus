extern crate sdl2;

use sdl2::event::{Event, EventType};
use sdl2::event::Event::Quit;
use sdl2::{Sdl, VideoSubsystem, EventPump};
use sdl2::video::Window;

struct LseWindow {
    sdl_window : Window,
    sdl_context : Sdl,
    sdl_video_subsystem : VideoSubsystem,
    sdl_event : EventPump
}

impl LseWindow{
    pub fn new(_title: &str, _width: u16, _height: u16) -> LseWindow {
        //Sdl2 Init
        let tmp_sdl_context : Sdl = sdl2::init().unwrap();
        //Video subsys
        let tmp_video_subsys : VideoSubsystem = tmp_sdl_context
            .video()
            .unwrap();
        //
        let tmp_sdl_event : EventPump = tmp_sdl_context.event_pump()
            .unwrap();
        //Building window
        let tmp_window : Window = tmp_video_subsys.window(_title, _width.into(), _height.into())
            .position_centered()
            .build()
            .unwrap();

         LseWindow{
            sdl_context: tmp_sdl_context, 
            sdl_video_subsystem: tmp_video_subsys,
            sdl_window: tmp_window,
            sdl_event: tmp_sdl_event
        }
    }

    pub fn is_running(&self) -> bool {

    }
}