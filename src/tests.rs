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
        assert_eq!(eta.total_time_elapsed, 0);
        assert_eq!(eta.paused, (None, 0));
    }

    #[test]
    fn test_eta_in_progress() {
        let eta = Eta::in_progress(10, TimeAcc::SEC, 5);
        assert_eq!(eta.tasks_count, 10);
        assert_eq!(eta.tasks_done, 5);
        assert_eq!(eta.time_accuracy, TimeAcc::SEC);
        assert_eq!(eta.total_time_elapsed, 0);
        assert_eq!(eta.paused, (None, 0));
    }

    #[test]
    fn test_eta_step() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.step();
        assert_eq!(eta.tasks_done, 1);
    }

    #[test]
    fn test_eta_elapsed() {
        let eta = Eta::new(10, TimeAcc::NANO);
        sleep(Duration::from_millis(1));
        assert!(eta.elapsed() > 0);
    }

    #[test]
    fn test_eta_total_elapsed_time() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        sleep(Duration::from_secs(1));
        eta.step();
        assert!(eta.total_time_elapsed > 0);
    }

    #[test]
    fn test_eta_pause() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.pause();
        assert!(eta.paused.0.is_some());
    }

    #[test]
    fn test_eta_resume() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        eta.pause();
        eta.resume();
        assert!(eta.paused.0.is_none());
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
        assert_eq!(eta.total_time_elapsed, 0);
    }

    #[test]
    fn test_eta_complex_paused() {
        let mut eta = Eta::new(10, TimeAcc::SEC);
        sleep(Duration::from_secs(1));
        eta.pause();
        sleep(Duration::from_secs(3));
        eta.resume();
        sleep(Duration::from_secs(7));
        eta.pause();
        sleep(Duration::from_secs(15));
        eta.step();
        assert_eq!(eta.total_time_elapsed, 2);
    }

    #[test]
    fn test_timeacc_display() {
        assert_eq!(format!("{}", TimeAcc::SEC), "s");
        assert_eq!(format!("{}", TimeAcc::MILLI), "ms");
        assert_eq!(format!("{}", TimeAcc::MICRO), "us");
        assert_eq!(format!("{}", TimeAcc::NANO), "ns");
    }
}