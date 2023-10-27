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
    #[opg(boolean)]
    only_collection_info: bool,
}

#[post("/metadata/refresh/")]
pub async fn refresh_metadata_by_nft(
    path: Json<RefreshMetadataParams>,
    meta_jrpc_service: web::Data<MetadataJrpcService>,
    meta_model_service: web::Data<MetadataModelService>,
) -> HttpResponse {
    let result = match path.0.nft {
        None => update_collection_metadata(&path, &meta_model_service, &meta_jrpc_service).await,
        Some(nft) => {
            update_nft_metadata(
                nft,
                &path.0.collection,
                &meta_model_service,
                &meta_jrpc_service,
            )
            .await
        }
    };

    respond_to_result(result)
}

async fn update_collection_metadata(
    path: &Json<RefreshMetadataParams>,
    meta_model_service: &web::Data<MetadataModelService>,
    meta_jrpc_service: &web::Data<MetadataJrpcService>,
) -> anyhow::Result<()> {
    data_reader::update_collections_meta(&path.0.collection, meta_model_service, meta_jrpc_service)
        .await?;

    if !path.only_collection_info {
        let nfts = meta_model_service
            .get_nfts_by_collection(&path.0.collection)
            .await?;

        for nft in nfts {
            update_nft_metadata(
                nft,
                &path.0.collection,
                meta_model_service,
                meta_jrpc_service,
            )
            .await?;
        }
    }

    Ok(())
}

async fn update_nft_metadata(
    nft: String,
    collection: &str,
    meta_model_service: &web::Data<MetadataModelService>,
    meta_jrpc_service: &web::Data<MetadataJrpcService>,
) -> anyhow::Result<()> {
    data_reader::update_nft_meta(
        &NftAddressData {
            nft,
            collection: collection.to_string(),
        },
        meta_model_service,
        meta_jrpc_service,
    )
    .await
}

fn respond_to_result(result: anyhow::Result<()>) -> HttpResponse {
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => {
            log::error!("calc metadata error {err}");
            HttpResponse::InternalServerError().finish()
        }
    }
}
