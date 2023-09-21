use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};
use data_reader::MetadataJrpcService;
use indexer_repo::meta::{MetadataModelService, NftAddressData};
use opg::OpgModel;
use serde::Deserialize;

#[derive(Deserialize, OpgModel)]
pub struct RefreshMetadataParams {
    #[opg(optional, string)]
    nft: Option<String>,
    #[opg(string)]
    collection: String,
}

#[post("/metadata/refresh/")]
pub async fn refresh_metadata_by_nft(
    path: Json<RefreshMetadataParams>,
    meta_jrpc_service: web::Data<MetadataJrpcService>,
    meta_model_service: web::Data<MetadataModelService>,
) -> HttpResponse {
    let result = match path.0.nft {
        None => {
            if let Err(e) = data_reader::update_collections_meta(
                &path.0.collection,
                &meta_model_service,
                &meta_jrpc_service,
            )
            .await
            {
                Err(e)
            } else {
                match meta_model_service
                    .get_nfts_by_collection(&path.0.collection)
                    .await
                {
                    Ok(nfts) => {
                        let mut result = Ok(());

                        for nft in nfts {
                            if let Err(e) = data_reader::update_nft_meta(
                                &NftAddressData {
                                    nft,
                                    collection: path.0.collection.clone(),
                                },
                                &meta_model_service,
                                &meta_jrpc_service,
                            )
                            .await
                            {
                                result = Err(e);
                                break;
                            }
                        }

                        result
                    }
                    Err(e) => Err(e),
                }
            }
        }
        Some(nft) => {
            data_reader::update_nft_meta(
                &NftAddressData {
                    nft,
                    collection: path.0.collection,
                },
                &meta_model_service,
                &meta_jrpc_service,
            )
            .await
        }
    };
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log::error!("calc metadata error {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
