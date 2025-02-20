use futures::join;
use reth_discv4::Discv4;
use reth_dns_discovery::DnsDiscoveryHandle;
use reth_primitives::NodeRecord;
use secp256k1::SecretKey;
use tokio::sync::mpsc;

use crate::crawler::listener::UpdateListener;

pub struct CrawlerService {
    updates: UpdateListener,
}

impl CrawlerService {
    pub async fn new(discv4: Discv4, dnsdisc: DnsDiscoveryHandle, key: SecretKey) -> Self {
        let (tx, rx) = mpsc::unbounded_channel::<Vec<NodeRecord>>();
        let updates = UpdateListener::new(discv4.clone(), dnsdisc.clone(), key, tx.clone()).await;
        Self { updates }
    }

    pub async fn run(self, save_to_json: bool) -> (eyre::Result<()>, eyre::Result<()>) {
        join!(
            self.updates.start_discv4(save_to_json),
            self.updates.start_dnsdisc(save_to_json),
        )
    }
}
