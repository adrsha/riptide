use libs_client::RTClient;
use libs_core::shared::RTShared;
use libs_server::RTServer;

pub struct Riptide {
    pub client: RTClient,
    pub server: RTServer,
    pub shared: RTShared,
    pub run:    fn(&Self)
}

impl Riptide {
    pub fn new() -> Self {
        Self {
            client: RTClient::default(),
            server: RTServer::default(),
            shared: RTShared::default(),
            run:    run_impl
        }
    }
}

impl Default for Riptide {
    fn default() -> Self { Self::new() }
}

pub fn run_impl(riptide: &Riptide) { println!("Running riptide") }
