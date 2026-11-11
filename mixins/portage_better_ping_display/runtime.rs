// Port of PingColors.java and ColorUtil.java. Copyright (c) 2020 Vlad Marica, MIT.
use std::cell::Cell;
use std::sync::atomic::{AtomicBool, Ordering};
static ENABLED: AtomicBool = AtomicBool::new(false);
thread_local! { static LAST_ENABLED: Cell<bool> = const { Cell::new(false) }; }
pub fn set_enabled(value: bool) {
    ENABLED.store(value, Ordering::Relaxed);
}
pub fn enabled() -> bool {
    ENABLED.load(Ordering::Relaxed)
}
pub fn changed() -> bool {
    let now = enabled();
    LAST_ENABLED.with(|last| last.replace(now) != now)
}
pub fn label(ping: i32) -> String {
    format!("{ping}ms")
}
pub fn color(ping: i32) -> u32 {
    if ping < 0 {
        return 0xFF535353;
    }
    let (a, b, t) = if ping < 150 {
        (0x00E676u32, 0xD6CD30u32, ping as f32 / 150.0)
    } else {
        (
            0xD6CD30u32,
            0xE53935u32,
            (ping.min(300) - 150) as f32 / 150.0,
        )
    };
    let mut rgb = 0xFF000000;
    for shift in [0, 8, 16] {
        let start = ((a >> shift) & 255) as f32;
        let end = ((b >> shift) & 255) as f32;
        rgb |= ((start + (end - start) * t).round() as u32) << shift;
    }
    rgb
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_palette_and_boundaries() {
        assert_eq!(color(-1), 0xFF535353);
        assert_eq!(color(0), 0xFF00E676);
        assert_eq!(color(75), 0xFF6BDA53);
        assert_eq!(color(150), 0xFFD6CD30);
        assert_eq!(color(300), 0xFFE53935);
        assert_eq!(color(i32::MAX), 0xFFE53935);
        assert_eq!(label(42), "42ms");
    }
}
