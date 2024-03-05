pub fn sys_time_to_seconds(time: std::time::SystemTime) -> u64 {
    let duration_since_epoch = time
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    return duration_since_epoch.as_secs();
}
