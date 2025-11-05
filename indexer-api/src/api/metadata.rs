use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::Deserialize;

use crate::server::AppState;

#[derive(Deserialize)]
pub struct RefreshMetadataParams {
    nft: Option<String>,
    collection: String,
    only_collection_info: bool,
}

pub async fn refresh_metadata_by_nft(
    State(state): State<AppState>,
    Json(params): Json<RefreshMetadataParams>,
) -> StatusCode {
    let result = match params.nft {
        None => update_collection_metadata(&params, &state).await,
        Some(nft) => update_nft_metadata(nft, &state).await,
    };

    respond_to_result(result)
}

async fn update_collection_metadata(
    params: &RefreshMetadataParams,
    state: &AppState,
) -> anyhow::Result<()> {
    state
        .meta_updater
        .update_collection_meta(&params.collection, params.only_collection_info, None)
        .await?;

    Ok(())
}

async fn update_nft_metadata(nft: String, state: &AppState) -> anyhow::Result<()> {
    state.meta_updater.update_nft_meta(&nft, None).await
}

fn respond_to_result(result: anyhow::Result<()>) -> StatusCode {
    match result {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            log::error!("calc metadata error {err}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
