#![allow(dead_code)]

mod tests;

use std::time::{Instant};

#[derive(Debug, Clone, PartialEq)]
pub struct Eta {
    tasks_count: usize,
    tasks_done: usize,
    recent_time: Instant,
    total_time_elapsed: usize,
    time_accuracy: TimeAcc,
    paused: Option<Instant>,
}

/* pub struct EtaMessageFormat {
    header: String,
    footing: String,
    accuracy: TimeAcc,
} */

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeAcc {
    SEC, MILLI, MICRO, NANO
}

impl Eta {
    fn create_instance(tasks_count: usize, time_accuracy: TimeAcc, tasks_done: usize) -> Eta {
        Eta {
            tasks_count,
            tasks_done,
            recent_time: Instant::now(),
            total_time_elapsed: 0,
            time_accuracy,
            paused: None,
        }
    }

    pub fn new (tasks_count: usize, time_accuracy: TimeAcc) -> Eta {
        Eta::create_instance(tasks_count, time_accuracy, 0)
    }

    pub fn in_progress(tasks_count: usize, time_accuracy: TimeAcc, tasks_done: usize) -> Eta {
        Eta::create_instance(tasks_count, time_accuracy, tasks_done)
    }

    /*fn pause(&self){
        //tbd
    }
    fn resume(&self){
        //tbd
    }*/

    pub fn step(&mut self) {
        self.tasks_done += 1;
        self.total_time_elapsed += self.elapsed();
        self.recent_time = Instant::now();
    }

    fn elapsed(&self) -> usize {
        match self.time_accuracy {
            TimeAcc::SEC => self.recent_time.elapsed().as_secs() as usize,
            TimeAcc::MILLI => self.recent_time.elapsed().as_millis() as usize,
            TimeAcc::MICRO => self.recent_time.elapsed().as_micros() as usize,
            TimeAcc::NANO => self.recent_time.elapsed().as_nanos() as usize
        }
    }

    pub fn progress(&self) -> f64 {
        (self.tasks_done as f64) / (self.tasks_count as f64)
    }

    pub fn time_remaining(&self) -> usize {
        ((self.tasks_count - self.tasks_done) as f64 * (self.total_time_elapsed as f64) / (self.tasks_done as f64))
            as usize
    }
}

impl std::fmt::Display for Eta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: {}% ({}{} remaining)", self.tasks_done, self.tasks_count, self.progress()*100.0, self.time_remaining(), self.time_accuracy)
    }
}

impl std::fmt::Display for TimeAcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeAcc::SEC => write!(f, "s"),
            TimeAcc::MILLI => write!(f, "ms"),
            TimeAcc::MICRO => write!(f, "us"),
            TimeAcc::NANO => write!(f, "ns"),
        }
    }
}