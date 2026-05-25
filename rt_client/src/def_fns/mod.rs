use rt_core::{
    errors::RTResult,
    shm::{
        RTShm,
        def_fns::seqlock::frame_read,
        layout::{RTShmFrame, RTShmMain}
    }
};

use crate::RTClient;

pub fn run_impl(client: &mut RTClient) -> RTResult<()> {
    match RTShm::<RTShmMain>::read(1) {
        Ok(shm) => {
            let main = (shm.get)(&shm);
            let keys = main.frame_keys;
            let mut shm_frames: Vec<RTShm<RTShmFrame>> = Vec::with_capacity(keys.len());

            for key in keys {
                if key == 0 {
                    continue;
                }
                match RTShm::<RTShmFrame>::read(key) {
                    Ok(shm) => {
                        shm_frames.push(shm);
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }
            }

            let frames: Vec<&RTShmFrame> =
                shm_frames.iter().map(|shm_f| (shm_f.get)(&shm_f)).collect();

            println!("{}", frames.len());

            for frame in frames {
                frame_read(frame, |fr| {
                    
                });
            }
        },
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}
