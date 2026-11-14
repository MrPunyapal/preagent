use tokio::process::Command;

#[test]
fn kill_on_drop_setting_must_be_honored() {
    let mut command = Command::new("sleep");
    assert!(!command.get_kill_on_drop(), "default must remain opt-out");
    command.kill_on_drop(true);
    assert!(command.get_kill_on_drop(), "opt-in must not be ignored");
    command.kill_on_drop(false);
    assert!(!command.get_kill_on_drop(), "opt-out must still work");
}

#[cfg(target_os = "linux")]
#[test]
fn dropping_an_opted_in_child_requests_termination() {
    use std::time::{Duration, Instant};
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let mut command = Command::new("sleep");
        command.arg("3").kill_on_drop(true);
        let child = command.spawn().expect("fixed sleep executable must start");
        let pid = child.id().expect("spawned child PID");
        drop(child);
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            let finished = match std::fs::read_to_string(format!("/proc/{pid}/stat")) {
                Err(_) => true,
                Ok(stat) => stat.rsplit_once(") ")
                    .map(|(_, tail)| tail.starts_with('Z'))
                    .unwrap_or(false),
            };
            if finished {
                break;
            }
            assert!(Instant::now() < deadline,
                "child with kill-on-drop remained alive after two seconds");
            std::thread::sleep(Duration::from_millis(20));
        }
    });
}
