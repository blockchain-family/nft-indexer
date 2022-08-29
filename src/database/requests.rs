use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, opg::OpgModel)]
#[serde(rename_all = "camelCase")]
pub struct AuctionDeployedRequest {
    // TODO
}
