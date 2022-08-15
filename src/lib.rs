//turn off warning for dead code
#![allow(dead_code)]
use std::time::{Instant};

struct Eta {
    count: usize,
    done: usize,
    times: Vec<usize>,
    last: Instant,
    total: usize,
    accuracy: TimeSteps,
    paused: Option<Instant>,
}
pub struct EtaMessageFormat {
    header: String,
    footing: String,
    accuracy: TimeSteps,
}

pub enum TimeSteps {
    SEC, MILLI, MICRO, NANO
}

impl Eta {
    fn new(count: usize, accuracy: TimeSteps, done: Option<usize>) -> Eta {
        Eta {
            count,
            done: match done {
                Some(d) => d,
                None => 0,
            },
            times: Vec::new(),
            last: Instant::now(),
            total: 0,
            accuracy,
            paused: None,
        }
    }

    fn pause(&self){
        //tbd
    }
    fn resume(&self){
        //tbd
    }
    fn step(&mut self) {
        let last = self.elapsed();
        self.times.push(last);
        self.last = Instant::now();
        
    }
    fn elapsed(&self) -> usize {
        match self.accuracy {
            TimeSteps::SEC => self.last.elapsed().as_secs() as usize,
            TimeSteps::MILLI => self.last.elapsed().as_millis() as usize,
            TimeSteps::MICRO => self.last.elapsed().as_micros() as usize,
            TimeSteps::NANO => self.last.elapsed().as_nanos() as usize
        }
    }
}