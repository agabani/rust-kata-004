use crate::command::Command;
use crate::job::Job;
use crate::pid::Pid;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Maintains a jobs lifecycle until signaled to terminate.
pub async fn event_loop(
    command: Command,
    pid: Pid,
    reload: Arc<AtomicBool>,
    terminate: Arc<AtomicBool>,
) {
    let (mut job, id) = start_job(&command);
    save_pid(&pid, id);

    while !termination_requested(&terminate) {
        if job_died(&mut job).await {
            stop_job(&mut job).await;
            let (new_job, id) = start_job(&command);
            save_pid(&pid, id);
            job = new_job;
        }

        if reload_requested(&reload) {
            let (new_job, id) = reload_job(job, &command).await;
            save_pid(&pid, id);
            job = new_job;
        }

        sleep().await;
    }

    stop_job(&mut job).await;
    delete_pid(&pid);
}

/// Deletes a pid.
fn delete_pid(pid: &Pid) {
    pid.reset().expect("Failed to delete PID.");
}

/// Saves a pid.
fn save_pid(pid: &Pid, id: u32) {
    pid.update(id).expect("Failed to save PID.");
}

/// Creates and starts a job.
fn start_job(command: &Command) -> (Job, u32) {
    let mut job = Job::new(command.create());
    job.start().expect("Failed to start job.");
    let id = job.id().expect("Failed to get PID.");
    (job, id)
}

/// Stops a job.
async fn stop_job(job: &mut Job) {
    job.stop().await.expect("Failed to stop job.");
}

/// Reloads a job.
#[cfg(target_family = "unix")]
async fn reload_job(job: Job, _command: &Command) -> (Job, u32) {
    job.reload();
    let id = job.id().expect("Failed to get PID.");
    (job, id)
}

/// Reloads a job.
#[cfg(target_family = "windows")]
async fn reload_job(mut job: Job, command: &Command) -> (Job, u32) {
    stop_job(&mut job).await;
    start_job(command)
}

/// Returns true if reload has been requested.
fn reload_requested(reload: &Arc<AtomicBool>) -> bool {
    reload.swap(false, Ordering::Relaxed)
}

/// Returns true if termination has been requested.
fn termination_requested(terminate: &Arc<AtomicBool>) -> bool {
    terminate.load(Ordering::Relaxed)
}

/// Returns true if the job has died.
async fn job_died(job: &mut Job) -> bool {
    job.status()
        .await
        .expect("Failed to query job status.")
        .is_some()
}

/// Sleeps for a constant duration.
async fn sleep() {
    tokio::time::sleep(Duration::from_millis(1000)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reload_requested_returns_true_if_requested() {
        let flag = Arc::new(AtomicBool::new(true));
        let requested = reload_requested(&flag);

        assert_eq!(true, requested);
        assert_eq!(
            false,
            flag.load(Ordering::Relaxed),
            "Flag should always reset to false."
        );
    }

    #[test]
    fn reload_requested_returns_false_if_not_requested() {
        let flag = Arc::new(AtomicBool::new(false));
        let requested = reload_requested(&flag);

        assert_eq!(false, requested);
        assert_eq!(
            false,
            flag.load(Ordering::Relaxed),
            "Flag should always reset to false."
        );
    }

    #[test]
    fn termination_requested_returns_true_if_requested() {
        let flag = Arc::new(AtomicBool::new(true));
        let requested = termination_requested(&flag);

        assert_eq!(true, requested);
        assert_eq!(
            true,
            flag.load(Ordering::Relaxed),
            "Flag should not change."
        );
    }

    #[test]
    fn termination_requested_returns_false_if_not_requested() {
        let flag = Arc::new(AtomicBool::new(false));
        let requested = termination_requested(&flag);

        assert_eq!(false, requested);
        assert_eq!(
            false,
            flag.load(Ordering::Relaxed),
            "Flag should not change."
        );
    }
}
