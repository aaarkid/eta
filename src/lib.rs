#![allow(dead_code)]
#![doc(html_favicon_url = "https://github.com/aaarkid/eta/blob/master/images/favicon.png")]
#![doc(html_logo_url = "https://github.com/aaarkid/eta/blob/master/images/logo.png")]
#![warn(missing_docs)]

//!Tracking progress on repetive tasks and measuring remaining times.
//!
//!# Usage
//! Add this to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! eta = "0.2.1"
//! ```
//! and this to your source code:
//! ```rust
//! use eta::{Eta,TimeAcc};
//! ```
//! 
//! # Example
//! ```rust
//! use eta::{Eta,TimeAcc};
//! 
//! fn calculate_square (number: usize) -> usize {
//!    number * number
//! }
//! 
//! fn main() {
//!     let count = 100;
//!     let numbers = Vec::from_iter(0..count);
//!     let mut eta = Eta::new(count, TimeAcc::MILLI);
//!     
//!     for number in numbers {
//!         calculate_square(number);
//!         eta.step();
//!         if (number % 10) == 0 {
//!             println!("{}", eta);
//!         }
//!     }
//! }

mod tests;

use std::time::{Instant};

#[derive(Debug, Clone, PartialEq, Eq)]
/// `Eta` is the main object which keep track of task count and elapsed times.
/// It implements several methods to display various information about the progress.
pub struct Eta {
    tasks_count: usize,
    tasks_done: usize,
    recent_time: Instant,
    total_time_elapsed: usize,
    time_accuracy: TimeAcc,
    time_paused: usize,
    paused: bool
}

#[derive(Clone, Debug, PartialEq, Eq)]
///`TimeAcc` is an enum for keeping track of the grade of accuracy for any information regarding time.
pub enum TimeAcc {
    ///SEC stands for seconds. 1 seconds has 1000 milliseconds.
    SEC,
    ///MILLI stands for milliseconds.
    MILLI,
    ///MICRO stands for microseconds. 1,000 microseconds make up 1 millisecond
    MICRO,
    ///NANO stands for nanoseconds. 1,000,000 nanoseconds make up 1 millisecond.
    NANO
}

impl Eta {
    fn create_instance(tasks_count: usize, time_accuracy: TimeAcc, tasks_done: usize) -> Eta {
        Eta {
            tasks_count,
            tasks_done,
            recent_time: Instant::now(),
            total_time_elapsed: 0,
            time_accuracy,
            time_paused: 0,
            paused: false,
        }
    }

    ///Creates a new `Eta` object with the given number of tasks and accuracy on time measurement.
    /// 
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// # }
    /// ```

    pub fn new (tasks_count: usize, time_accuracy: TimeAcc) -> Eta {
        Eta::create_instance(tasks_count, time_accuracy, 0)
    }

    /// Pauses time measurement for the object. Resuming time measurement is done by calling `resume()`.
    /// Time between pauses is kept track of and not reset. You can step `Eta` while it's paused.
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # use std::time::Duration;
    /// # use std::thread::sleep;
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let mut eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// sleep(Duration::from_secs(1)); // one second elapses
    /// eta.pause();
    /// sleep(Duration::from_secs(1)); // this other second isn't kept track of
    /// eta.step(); // time elapsed will account for approx. 1 second here
    /// eta.resume(); // does nothing as eta is resumed automatically right after eta.step()
    /// # }
    /// ```
    pub fn pause(&mut self) {
        if !self.paused {
            self.paused = true;
            self.time_paused += self.elapsed();
        }
    }

    /// Resumes time measurement for the object. Must be called after pausing with eta.pause(),
    /// otherwise it will do nothing. Object is resumed automatically after eta.step().
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # use std::time::Duration;
    /// # use std::thread::sleep;
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let mut eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// eta.pause(); // eta is paused
    /// sleep(Duration::from_secs(1)); // this second is NOT kept track of
    /// eta.resume(); // resumes the object
    /// sleep(Duration::from_secs(1)); // this other second is kept track of
    /// eta.step(); // time elapsed will account for approx. 1 seconds here
    /// # }
    /// ```
    pub fn resume(&mut self) {
        if self.paused {
            self.recent_time = Instant::now();
            self.paused = false;
        }
    }

    /// Steps the eta object by one task and does all the time calculations at the time of being called.
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let mut eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// for something in 0..count_of_tasks {
    ///     // do something
    ///     eta.step();
    ///     println!("{}", eta);
    /// }
    /// # }
    /// ```
    pub fn step(&mut self) {
        self.tasks_done += 1;
        if !self.paused {
            self.total_time_elapsed += self.elapsed();
        }
        self.total_time_elapsed += self.time_paused;
        self.recent_time = Instant::now();
        self.paused = false;
        self.time_paused = 0;
    }

    fn elapsed(&self) -> usize {
        match self.time_accuracy {
            TimeAcc::SEC => self.recent_time.elapsed().as_secs() as usize,
            TimeAcc::MILLI => self.recent_time.elapsed().as_millis() as usize,
            TimeAcc::MICRO => self.recent_time.elapsed().as_micros() as usize,
            TimeAcc::NANO => self.recent_time.elapsed().as_nanos() as usize
        }
    }

    /// Returns the portion of tasks that have been completed as a double between 0 and 1.
    /// Multiply by 100 to get the according percentage.
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let mut eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// for something in 0..count_of_tasks {
    ///     // do something
    ///     eta.step();
    ///     println!("{}% of the job is done", eta.progress()*100 as f64);
    /// }
    /// # }
    /// ```
    pub fn progress(&self) -> f64 {
        (self.tasks_done as f64) / (self.tasks_count as f64)
    }

    /// Returns the estimated amount of time left for the job.
    /// 
    /// # Example
    /// ```rust
    /// # use eta::{Eta,TimeAcc};
    /// # fn main () {
    /// # let count_of_tasks = 100;
    /// let mut eta = Eta::new(count_of_tasks, TimeAcc::MILLI);
    /// for something in 0..count_of_tasks {
    ///     // do something
    ///     eta.step();
    ///     println!("Job will be finished in {}s.", eta.time_remaining()/1000);
    /// }
    /// # }
    /// ```
    pub fn time_remaining(&self) -> usize {
        ((self.tasks_count - self.tasks_done) as f64 * (self.total_time_elapsed as f64) / (self.tasks_done as f64))
            as usize
    }
}

#[doc(hidden)]
impl std::fmt::Display for Eta {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}: {}% ({}{} remaining)", self.tasks_done, self.tasks_count, self.progress()*100.0, self.time_remaining(), self.time_accuracy)
    }
}

#[doc(hidden)]
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