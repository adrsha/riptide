use std::{
    fmt::format,
    sync::atomic::{AtomicU64, Ordering}
};

use crate::{errors::RTResult, shm::layout::RTShmFrame};

pub fn frame_write_begin(frame: &RTShmFrame) {
    let seq = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };
    seq.fetch_add(1, Ordering::AcqRel);
}

pub fn frame_write_end(frame: &RTShmFrame) {
    let seq = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };
    seq.fetch_add(1, Ordering::Release);
}

/// Reads a consistent snapshot of `RTShmFrame` using a seqlock-style protocol.
///
/// The `read_fn` closure is executed on a *best-effort snapshot view* of the frame.
/// The underlying memory may be concurrently modified, so the data passed to the
/// closure can be temporarily inconsistent or torn.
///
/// This function does NOT guarantee stability during `read_fn`. Instead, it detects
/// concurrent modification by comparing a sequence counter before and after the read.
///
/// # Contract
/// - If this function returns `true`, the closure observed a consistent snapshot.
/// - If it returns `false`, the snapshot was invalid and the caller MUST retry.
///
/// # Important
/// The closure must treat all data as ephemeral read-only snapshot data and must not
/// assume stability across calls or reuse references beyond its scope.
pub fn frame_read<F, R>(frame: &RTShmFrame, read_fn: F) -> RTResult<R>
where
    F: FnOnce(&RTShmFrame) -> R
{
    let seq_ptr = unsafe { &*(&frame.seq as *const u64 as *const AtomicU64) };

    let seq1 = seq_ptr.load(Ordering::Acquire);
    if seq1 & 1 != 0 {
        return Err(crate::errors::RTErrors::InvalidValue {
            field:  String::from("seq"),
            reason: String::from("seq is odd")
        });
    }

    let res = read_fn(frame);

    let seq2 = seq_ptr.load(Ordering::Acquire);
    if seq1 == seq2 {
        return Ok(res);
    }
    else {
        return Err(crate::errors::RTErrors::InvalidValue {
            field:  String::from("seq"),
            reason: String::from("is inconsistent")
        });
    }
}
