//turn off warning for dead code
#![allow(dead_code)]
use std::time::{Instant};

#[derive(Debug, Clone)]
struct Eta {
    count: usize,
    done: usize,
    last: Instant,
    total: usize,
    accuracy: TimeAcc,
    paused: Option<Instant>,
}

/* pub struct EtaMessageFormat {
    header: String,
    footing: String,
    accuracy: TimeAcc,
} */

#[derive(Clone)]
pub enum TimeAcc {
    SEC, MILLI, MICRO, NANO
}

impl Eta {
    fn create_instance(count: usize, accuracy: TimeAcc, done: usize) -> Eta {
        Eta {
            count,
            done,
            last: Instant::now(),
            total: 0,
            accuracy,
            paused: None,
        }
    }

    pub fn new (count: usize, accuracy: TimeAcc) -> Eta {
        Eta::create_instance(count, accuracy, 0)
    }

    pub fn in_progress(count: usize, accuracy: TimeAcc, done: usize) -> Eta {
        Eta::create_instance(count, accuracy, done)
    }

    /*fn pause(&self){
        //tbd
    }
    fn resume(&self){
        //tbd
    }*/

    pub fn step(&mut self) {
        self.total += self.elapsed();
        self.last = Instant::now();
    }

    fn elapsed(&self) -> usize {
        match self.accuracy {
            TimeAcc::SEC => self.last.elapsed().as_secs() as usize,
            TimeAcc::MILLI => self.last.elapsed().as_millis() as usize,
            TimeAcc::MICRO => self.last.elapsed().as_micros() as usize,
            TimeAcc::NANO => self.last.elapsed().as_nanos() as usize
        }
    }

    pub fn progress(&self) -> f64 {
        (self.done as f64) / (self.count as f64)
    }

    pub fn time_remaining(&self) -> usize {
        let remaining = (self.count - self.done) as f64 * (self.total as f64) / (self.done as f64);
        remaining as usize
    } 
}

impl std::fmt::Display for Eta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: {}%", self.done, self.count, self.progress()*100.0)
    }
}

impl std::fmt::Debug for TimeAcc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeAcc::SEC => write!(f, "s"),
            TimeAcc::MILLI => write!(f, "ms"),
            TimeAcc::MICRO => write!(f, "us"),
            TimeAcc::NANO => write!(f, "ns"),
        }
    }
}