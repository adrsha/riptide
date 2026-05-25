use rt_core::shm::layout::RTShmFrameView;

pub trait RTLoopDriver {
    fn run<F>(&self, handler: F)
    where
        F: FnMut(&mut RTShmFrameView);
}
