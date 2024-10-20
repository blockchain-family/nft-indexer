use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};
use data_reader::MetaUpdater;
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
    meta_updater: web::Data<MetaUpdater>,
) -> HttpResponse {
    let result = match path.0.nft {
        None => update_collection_metadata(&path, meta_updater).await,
        Some(nft) => update_nft_metadata(nft, meta_updater).await,
    };

    respond_to_result(result)
}

async fn update_collection_metadata(
    path: &Json<RefreshMetadataParams>,
    meta_updater: web::Data<MetaUpdater>,
) -> anyhow::Result<()> {
    meta_updater
        .update_collection_meta(&path.0.collection, path.only_collection_info, None)
        .await?;

    Ok(())
}

async fn update_nft_metadata(
    nft: String,
    meta_updater: web::Data<MetaUpdater>,
) -> anyhow::Result<()> {
    meta_updater.update_nft_meta(&nft, None).await
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
