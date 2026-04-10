#[derive(bitcode::Encode, bitcode::Decode)]
pub struct RTSignal {
    pub server_id:   usize,
    pub signal_type: RTCommand
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub enum RTCommand {
    Buffer(RTBufferCommand),
    Cluster(RTClusterCommand),
}

#[derive(bitcode::Encode, bitcode::Decode)]
pub enum RTBufferCommand {}

#[derive(bitcode::Encode, bitcode::Decode)]
pub enum RTClusterCommand {
    OpenCluster,
    CloseCluster,
    SplitFrame,
    CloseFrame,
    AssignBuffer
}
