use dotenv::dotenv;
use log::{error, info, warn};
use web3::futures::TryStreamExt;
use web3::transports::WebSocket;
use web3::types::TransactionId;

fn get_node_endpoint() -> String {
    std::env::var("WSS_NODE_ENDPOINT").expect("No WSS_NODE_ENDPOINT found")
}

fn get_private_key() -> String {
    let dummy_private_key = "cb32a531d4c06247c507aff194e57e1259fb03b6361bf9f22173f2223484ccfb";

    match std::env::var("PRIVATE_KEY") {
        Ok(key) => key,
        Err(_) => {
            warn!("No PRIVATE_KEY found, using dummy key");
            dummy_private_key.to_string()
        }
    }
}

#[tokio::main]
async fn main() -> web3::Result {
    dotenv().ok();
    env_logger::init();

    let wss_node_endpoint = get_node_endpoint();

    let sub_transport = WebSocket::new(wss_node_endpoint.as_str()).await?;
    let web3 = web3::Web3::new(sub_transport);

    let mut pending_transactions = web3
        .eth_subscribe()
        .subscribe_new_pending_transactions()
        .await?;

    while let Some(pending_transaction_hash) = pending_transactions.try_next().await? {
        let pth = TransactionId::from(pending_transaction_hash);

        let res = web3.eth().transaction(pth).await;
        match res {
            Ok(opt_txn) => match opt_txn {
                Some(txn) => info!("{:?}", txn),
                None => warn!("could not find transaction for now"),
            },
            Err(e) => error!("{:?}", e),
        }
    }

    Ok(())
}
