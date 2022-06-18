use std::sync::Mutex;
use std::thread;

enum LseThreadReturn<T> {
    None,
    Some(T)
}

trait LseThreadFunction{
    fn run<T>() -> LseThreadReturn<T>;
}

pub struct LseThreads<T>{
    m_mutex:    Mutex<LseThreadReturn<T>>,
    m_start_finished_mutex: Mutex<LseThreadReturn<T>>,
    m_join:     bool,
    m_retval:   LseThreadReturn<T>,
}
