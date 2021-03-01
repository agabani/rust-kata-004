use kube::api::{Meta, Patch, PatchParams};
use kube::Api;
use kube_runtime::controller::{Context, ReconcilerAction};

use super::data::Data;
use super::error::Error;
use super::tor_hidden_service_spec::TorHiddenService;
use super::tor_hidden_service_status::TorHiddenServiceStatus;

#[tracing::instrument(skip(ctx))]
pub async fn reconcile(
    tor_hidden_service: TorHiddenService,
    ctx: Context<Data>,
) -> Result<ReconcilerAction, Error> {
    let name = Meta::name(&tor_hidden_service);

    tracing::info!(
        "Reconcile TorHiddenService {}: {:?}",
        name,
        tor_hidden_service
    );

    // create client
    let client = ctx.get_ref().client.clone();
    let namespace = Meta::namespace(&tor_hidden_service).expect("Failed to get service namespace.");
    let api: Api<TorHiddenService> = Api::namespaced(client, &namespace);

    // calculate new status
    let patch = Patch::Apply(serde_json::json!({
        "apiVersion": "agabani.rust-kata-004/v1",
        "kind": "TorHiddenService",
        "status": TorHiddenServiceStatus {
            hostname: Some("example".to_string())
        }
    }));
    let patch_params = PatchParams::apply("cntrlr").force();
    let _o = api
        .patch_status(&name, &patch_params, &patch)
        .await
        .expect("TODO: error handling");

    Ok(ReconcilerAction {
        requeue_after: Some(std::time::Duration::from_secs(1800)),
    })
}
