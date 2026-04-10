pub mod gpu;
pub mod tty;

pub enum RTSession {
    Gpu(gpu::GpuSession),
    Tty(tty::TtySession)
}
