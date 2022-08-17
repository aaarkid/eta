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
        assert_eq!(eta.paused, None);
    }

    #[test]
    fn test_eta_in_progress() {
        let eta = Eta::in_progress(10, TimeAcc::SEC, 5);
        assert_eq!(eta.tasks_count, 10);
        assert_eq!(eta.tasks_done, 5);
        assert_eq!(eta.time_accuracy, TimeAcc::SEC);
        assert_eq!(eta.total_time_elapsed, 0);
        assert_eq!(eta.paused, None);
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
    fn test_eta_elapsed_time() {
        let mut eta = Eta::new(10, TimeAcc::NANO);
        sleep(Duration::from_millis(1));
        eta.step();
        assert!(eta.total_time_elapsed > 0);
    }
}