use rt_core::shm::{RTShmMut, layout::RTShmFrame};

pub type RTProcessId = slotmap::DefaultKey;
pub struct RTProcess {
    pub shm: RTShmMut<RTShmFrame>
}

impl RTProcess {
    pub fn new(shm_key: u64) -> Self {
        match RTShmMut::<RTShmFrame>::create(shm_key) {
            Ok(shm) => {
                Self {
                    shm
                }
            },
            Err(err) => panic!("{}", err)
        }
    }
}
