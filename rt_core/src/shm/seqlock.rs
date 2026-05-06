use std::sync::atomic::{AtomicU64, Ordering};

use crate::shm::layout::RTShmFrame;

pub fn frame_write_begin(frame: &RTShmFrame) {
    let seq = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };
    seq.fetch_add(1, Ordering::Relaxed);
}

pub fn frame_write_end(frame: &RTShmFrame) {
    let seq = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };
    seq.fetch_add(1, Ordering::Release);
}

pub fn frame_read_is_clean<F>(frame: &RTShmFrame, read_fn: F) -> bool
where
    F: FnOnce(&RTShmFrame)
{
    let seq_ptr = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };

    let seq1 = seq_ptr.load(Ordering::Acquire);
    if seq1 & 1 != 0 {
        return false;
    }

    read_fn(frame);

    let seq2 = seq_ptr.load(Ordering::Acquire);
    seq1 == seq2
}
