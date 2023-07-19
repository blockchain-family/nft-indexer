use anyhow::Result;
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

pub struct MetadataJrpcService {
    jrpc_client: JrpcClient,
}

impl MetadataJrpcService {
    pub fn new(jrpc_client: JrpcClient) -> Self {
        Self { jrpc_client }
    }

    pub async fn fetch_metadata(&self, address: &MsgAddressInt) -> Result<serde_json::Value> {
        rpc::retrier::Retrier::new(|| {
            Box::pin(rpc::get_json(address.clone(), self.jrpc_client.clone()))
        })
        .attempts(1)
        .trace_id(format!(
            "fetch metadata {}",
            address.address().as_hex_string()
        ))
        .run()
        .await
    }
}
