extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub struct LseWindow<'a>{
    title: &'a str,
    width: u16,
    height: u16,
    sdl_instance:       sdl2::Sdl,
    video_subsystem:    sdl2::VideoSubsystem,
    window_instance:    sdl2::video::Window
}

impl<'a> LseWindow<'a> {
    pub fn new(_title: &'a str, _width: u16, _height: u16) -> Result<LseWindow<'a>, Box <dyn std::error::Error>>{
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window(_title, _width as u32, _height as u32)
            .position_centered()
            .build()
            .unwrap();

        Ok(LseWindow{
            title:  _title,
            width:  _width,
            height: _height,
            sdl_instance: sdl_context,
            video_subsystem: video_subsystem,
            window_instance: window
        })
    }

    pub fn swap_buffers(&self) {

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
