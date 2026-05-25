use riptide::rt_client::RTClient;

fn main() {
    let mut client = RTClient::default();
    if let Err(result) = (client.run)(&mut client){
        result.log();
    }
}
