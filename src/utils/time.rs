use std::time::{SystemTime, UNIX_EPOCH};


pub fn unixtime() -> u32 {
    let now = SystemTime::now();
    if let Ok(duration_since_epoch) = now.duration_since(UNIX_EPOCH) {
        // duration_since_epoch.as_secs() 会得到秒数
        let timestamp = duration_since_epoch.as_secs();
        return timestamp as u32;
    } else {
        // 系统时间在 UNIX_EPOCH 之前，这在正常情况下不会发生
        eprintln!("系统时间在 Unix 时间纪元之前。");
        return 0;
    }
}