//! Cross platform system time access and FPS counters.

use crate::{get_context, text::draw_text, with_context};

/// Draws the current FPS on the screen. For extra customization, please use `draw_text` instead.
pub fn draw_fps() {
    draw_text(&format!("FPS: {}", get_fps()), 0., 16., 32., crate::WHITE);
}

/// Returns current FPS
pub fn get_fps() -> i32 {
    with_context(|context| (1. / context.frame_time) as i32)
}

/// Returns duration in seconds of the last frame drawn
pub fn get_frame_time() -> f32 {
    with_context(|context| context.frame_time as f32)
}

/// Returns elapsed wall-clock time in seconds since start
///
/// Note that as real world time progresses during computation,
/// the value returned will change. Therefore if you want
/// your game logic update to happen at the same *in-game* time
/// for all game objects, you should call this function once
/// save the value and reuse it throughout your code.
pub fn get_time() -> f64 {
    with_context(|context| miniquad::date::now() - context.start_time)
}
