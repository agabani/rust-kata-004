#![allow(clippy::field_reassign_with_default)]

use super::tor_hidden_service_status::TorHiddenServiceStatus;

#[derive(
    Clone, Debug, kube::CustomResource, schemars::JsonSchema, serde::Serialize, serde::Deserialize,
)]
#[kube(
    kind = "TorHiddenService",
    group = "agabani.rust-kata-004",
    version = "v1",
    namespaced,
    status = "TorHiddenServiceStatus"
)]
pub struct TorHiddenServiceSpec {
    name: String,
    host: String,
    port: u16,
}
