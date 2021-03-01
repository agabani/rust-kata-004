use std::future::Future;
use std::pin::Pin;

use futures::{FutureExt, StreamExt};
use kube::api::ListParams;
use kube::{Api, Client};
use kube_runtime::controller::Context;
use kube_runtime::Controller;

use super::data::Data;
use super::tor_hidden_service_spec::TorHiddenService;
use super::{error_policy, reconcile};

#[derive(Clone)]
pub struct Manager {}

impl Manager {
    pub(crate) async fn new(
        client: Client,
    ) -> (Self, Pin<Box<dyn Future<Output = ()> + Send + 'static>>) {
        let context = Context::new(Data {
            client: client.clone(),
        });

        let api = Api::<TorHiddenService>::all(client);

        let drainer = Controller::new(api, ListParams::default())
            .run(reconcile::reconcile, error_policy::error_policy, context)
            .filter_map(|x| async move { std::result::Result::ok(x) })
            .for_each(|o| {
                tracing::info!("Reconciled {:?}", o);
                futures::future::ready(())
            })
            .boxed();

        (Self {}, drainer)
    }
}
