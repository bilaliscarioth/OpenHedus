extern crate sdl2;

use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

pub struct LseWindow{
    sdl_instance:       Sdl,
    video_subsystem:    VideoSubsystem,
}

impl<'a> LseWindow {
    pub fn new() -> Result<LseWindow, Box <dyn std::error::Error>>{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        Ok(LseWindow{
            sdl_instance: sdl_context,
            video_subsystem: video_subsystem,
        })
    }

    pub fn create_window(&self, _title: &'a str, _width: u16, _height: u16) -> Window {
        self.video_subsystem.window(_title, _width as u32, _height as u32)
            .position_centered()
            .build()
            .unwrap()

    }

    pub fn get_events(&self)  -> sdl2::EventPump {
        self.sdl_instance.event_pump().unwrap()
    }

    pub fn quit(&self) -> bool {
        let mut events = self.get_events();
        let event = events.poll_iter().last();

        match event {
            Some(Event::Quit {..}) => {
                return true
            },
            _ => {}
        }
        false
    }
}
