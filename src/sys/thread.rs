use std::thread::*;
use std::sync::{Mutex,Arc};

use crate::sys::logging::{LseLogging};
use crate::sys::log::{config, level};

pub struct LseThreads{
    thr_info:   Builder,
    thr_obj:    Option<JoinHandle<()>>,
    function:   fn() -> (),
    running:    Arc<bool>,
    log:        LseLogging
}

impl LseThreads {

    pub fn new(name: String, stacksize: usize, function: fn() -> ()) -> LseThreads{

        let m_thread_info = Builder::new()
            .name(name)
            .stack_size(stacksize);

        let running = Arc::new(false);

        LseThreads{
            thr_info:   m_thread_info,
            thr_obj:    None,
            function: function,
            running: running,
            log: LseLogging::new(
                    2,
                    "./log/log.txt".to_string(),
                    "{time} {thread_name} | {level_name} {message}".to_string(),
                    "{ascii}".to_string(),
                    false
            )
        }
    }

    pub fn start(&mut self) -> bool {
        match Arc::get_mut(&mut self.running) {
            Some(_bool) => {
                self.running = Arc::new(true);
                self.thr_obj = Some(LseThreads::run(self.function));

                true
            }
            None => {
                false
            }
        }
    }

    pub fn stop(mut self) -> bool {
        self.running = Arc::new(false);
        LseThreads::stop_thread(self.thr_obj)
    }

    fn stop_thread(thr_obj : Option<JoinHandle<()>>) -> bool {
        match thr_obj {
            Some(thread) => {
                match thread.join() {
                    Ok(a) =>{
                        true
                    },
                    Err(b) => {
                        false
                    }
                }
            },
            None => { false }
        }

    }

    fn run(function: fn() -> ()) -> JoinHandle<()> {
        spawn(function)
    }

    pub fn is_running(&mut self) -> bool {
        match Arc::get_mut(&mut self.running) {
            Some (_bool) => {
                return *_bool
            },
            None => { return false }
        }
    }
}
