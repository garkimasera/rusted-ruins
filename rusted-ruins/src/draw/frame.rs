
use std::cell::Cell;
use common::obj::Img;

thread_local!(static FRAME_COUNT: Cell<u32> = Cell::new(0));

/// Add 1 to frame counter
pub fn next_frame() {
    FRAME_COUNT.with(|frame_count| {
        let a = frame_count.get();
        frame_count.set(a.wrapping_add(1));
    });
}

/// Calculate which animation frame of image will be used
pub fn calc_frame(img: &Img) -> u32 {
    if img.n_anim_frame == 1 { return 0; }
    
    let frame_count = FRAME_COUNT.with(|frame_count| frame_count.get());
    let a = frame_count % (img.duration * img.n_frame);
    let n = a / img.duration;
    debug_assert!(n < img.n_frame);
    n
}
