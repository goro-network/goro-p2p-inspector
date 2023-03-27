#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub(crate) mod logging;
pub(crate) mod networks;
pub(crate) mod options;
pub(crate) mod peer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logging::init_logger();

    let lookup_client = peer::PeerLookupClient::create_from_options();

    match lookup_client.do_lookup().await {
        Err(error_message) => {
            logging::log_error!("{error_message}");
        }
        Ok(lookup_result) => {
            logging::log_info!("Peer information from lookup\n********\n{lookup_result}********");
        }
    }

    Ok(())
}
