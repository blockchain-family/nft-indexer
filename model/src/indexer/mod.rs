pub mod consumer;
pub mod events;
pub mod record_build_utils;
pub mod traits;

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_meta() {
        let meta: serde_json::Value = serde_json::from_str(
      r#"
      {
        "output": {
          "json": "{\"type\":\"Basic NFT\",\"name\":\"The Everscale Anniversary NFT\",\"description\":\"Everscale is celebrating its second anniversary! We’re grateful to our community for being with us since the beginning, helping us with ideas, and actively participating in the development process. That’s why we decided to create a unique commemorative NFT for our community.\",\"preview\":{\"source\":\"https://ipfs.itgold.io/ipfs/QmfSv4UB62dopa3YnPUkbejChLnx95Vdn1YWH44Q6FQDw4\",\"width\":268,\"height\":268,\"size\":73631,\"mimetype\":\"image/png\",\"format\":\"png\"},\"files\":[{\"source\":\"https://ipfs.itgold.io/ipfs/QmfSv4UB62dopa3YnPUkbejChLnx95Vdn1YWH44Q6FQDw4\",\"width\":268,\"height\":268,\"size\":73631,\"mimetype\":\"image/png\",\"format\":\"png\"}],\"external_url\":\"https://grandbazar.io/collection/everscale-anniversary\"}"
        },
        "code": 0
      }
      "#
    ).unwrap();

        let json: serde_json::Value = serde_json::from_str(
            meta.get("output")
                .unwrap()
                .get("json")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap();

        println!("{:#?}", json);
    }
}
