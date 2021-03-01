use kube_runtime::controller::{Context, ReconcilerAction};

use super::data::Data;
use super::error::Error;

pub fn error_policy(error: &Error, _ctx: Context<Data>) -> ReconcilerAction {
    tracing::warn!("reconcile failed: {}", error);
    ReconcilerAction {
        requeue_after: Some(std::time::Duration::from_secs(360)),
    }
}
