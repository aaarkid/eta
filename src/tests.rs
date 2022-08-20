#[cfg(test)]
mod tests {
    use crate::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_eta_new() {
        let eta = Eta::new(10, TimeAcc::SEC);
        assert_eq!(eta.tasks_count, 10);
        assert_eq!(eta.tasks_done, 0);
        assert_eq!(eta.time_accuracy, TimeAcc::SEC);
        assert_eq!(eta.time_elapsed, 0);
        assert_eq!(eta.paused, false);
        assert_eq!(eta.time_paused, 0);
    }

    #[test]
    fn test_eta_step() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.step();
        assert_eq!(eta.tasks_done, 1);
    }

    #[test]
    fn test_eta_elapsed() {
        let eta1 = Eta::new(10, TimeAcc::SEC);
        let eta2 = Eta::new(10, TimeAcc::MILLI);
        let eta3 = Eta::new(10, TimeAcc::MICRO);
        let eta4 = Eta::new(10, TimeAcc::NANO);
        sleep(Duration::from_secs(1));
        assert!(eta1.step_elapsed() > 0);
        assert!(eta2.step_elapsed() > 0);
        assert!(eta3.step_elapsed() > 0);
        assert!(eta4.step_elapsed() > 0);
    }

    #[test]
    fn test_eta_total_elapsed_time() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        sleep(Duration::from_secs(1));
        eta.step();
        assert!(eta.time_elapsed > 0);
    }

    #[test]
    fn test_eta_pause() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.pause();
        assert!(eta.paused);
    }

    #[test]
    fn test_eta_resume() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.pause();
        eta.resume();
        assert!(!eta.paused);
    }

    #[test]
    fn test_eta_progress() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.step();
        assert_eq!(eta.progress(), 0.1);
    }

    #[test]
    fn test_eta_time_remaining() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        sleep(Duration::from_secs(1));
        eta.step();
        assert_eq!(eta.time_remaining(), 9);
    }

    #[test]
    fn test_eta_time_remaining_paused() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.pause();
        sleep(Duration::from_secs(1));
        eta.resume();
        eta.step();
        assert_eq!(eta.time_elapsed, 0);
    }

    #[test]
    fn test_eta_complex_paused() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        sleep(Duration::from_secs(1));
        eta.pause();
        sleep(Duration::from_secs(1));
        eta.resume();
        sleep(Duration::from_secs(1));
        eta.pause();
        sleep(Duration::from_secs(1));
        eta.step();
        assert_eq!(eta.time_elapsed, 2);
    }

    #[test]
    fn test_timeacc_display() {
        assert_eq!(format!("{}", TimeAcc::SEC), "s");
        assert_eq!(format!("{}", TimeAcc::MILLI), "ms");
        assert_eq!(format!("{}", TimeAcc::MICRO), "µs");
        assert_eq!(format!("{}", TimeAcc::NANO), "ns");
    }
    
    #[test]
    fn test_eta_display() {
        let mut eta1 = Eta::new(10, TimeAcc::SEC);
        let mut eta2 = Eta::new(100, TimeAcc::SEC);
        let mut eta3 = Eta::new(61, TimeAcc::SEC);
        let mut eta4 = Eta::new(1, TimeAcc::MILLI);
        let mut eta5 = Eta::new(1, TimeAcc::MICRO);
        let mut eta6 = Eta::new(1, TimeAcc::NANO);
        sleep(Duration::from_secs(1));
        eta1.step();
        eta2.step();
        eta3.step();
        eta4.step();
        eta5.step();
        eta6.step();
        assert_eq!(eta1.to_string(), "1/10: 10% (9s remaining)".to_owned());
        assert_eq!(eta2.to_string(), "1/100: 1% (1m 39s remaining)".to_owned());
        assert_eq!(eta3.to_string(), "1/61: 2% (1m remaining)".to_owned());
        assert_eq!(eta4.to_string(), "1/1: 100% (0s remaining)".to_owned());
        assert_eq!(eta5.to_string(), "1/1: 100% (0ms remaining)".to_owned());
        assert_eq!(eta6.to_string(), "1/1: 100% (0µs remaining)".to_owned());
    }
}