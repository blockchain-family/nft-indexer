use anyhow::Result;
use ton_block::MsgAddressInt;
use transaction_consumer::JrpcClient;

#[derive(Clone)]
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
        .trace_id(format!("fetch metadata {}", address))
        .run()
        .await
    }

    pub async fn get_collection_owner(&self, collection: &MsgAddressInt) -> Result<String> {
        rpc::retrier::Retrier::new(|| {
            Box::pin(rpc::owner(collection.clone(), self.jrpc_client.clone()))
        })
        .attempts(1)
        .trace_id(format!("collection owner {}", collection))
        .run()
        .await
    }
}
