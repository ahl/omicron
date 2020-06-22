/*!
 * Interface for making API requests to a Sled Agent
 *
 * This should be replaced with a client generated from the OpenAPI spec
 * generated by the server.
 */

use crate::api_error::ApiError;
use crate::api_model::ApiDiskRuntimeState;
use crate::api_model::ApiDiskStateRequested;
use crate::api_model::ApiInstanceRuntimeState;
use crate::api_model::ApiInstanceRuntimeStateRequested;
use crate::api_model::DiskEnsureBody;
use crate::api_model::InstanceEnsureBody;
use crate::http_client::HttpClient;
use async_trait::async_trait;
use http::Method;
use hyper::Body;
use slog::Logger;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

/** Client for a sled agent */
pub struct SledAgentClient {
    /**
     * sled agent's unique id (used by callers to track `SledAgentClient`
     * objects)
     */
    pub id: Uuid,
    /** sled agent server address */
    pub service_address: SocketAddr,
    /** underlying HTTP client */
    client: HttpClient,
}

impl SledAgentClient {
    /**
     * Create a new sled agent client to make requests to the sled agent running
     * at `server_addr`.
     */
    pub fn new(
        id: &Uuid,
        server_addr: SocketAddr,
        log: Logger,
    ) -> SledAgentClient {
        SledAgentClient {
            id: *id,
            service_address: server_addr,
            client: HttpClient::new("sled agent", server_addr, log),
        }
    }

    /**
     * Idempotently ensures that the given API Instance exists on this server in
     * the given runtime state (described by `target`).
     */
    pub async fn instance_ensure(
        self: &Arc<Self>,
        instance_id: Uuid,
        initial_runtime: ApiInstanceRuntimeState,
        target: ApiInstanceRuntimeStateRequested,
    ) -> Result<ApiInstanceRuntimeState, ApiError> {
        let path = format!("/instances/{}", instance_id);
        let body = Body::from(
            serde_json::to_string(&InstanceEnsureBody {
                initial_runtime,
                target,
            })
            .unwrap(),
        );
        let mut response =
            self.client.request(Method::PUT, path.as_str(), body).await?;
        /* TODO-robustness handle 300-level? */
        assert!(response.status().is_success());
        let value = self
            .client
            .read_json::<ApiInstanceRuntimeState>(
                &self.client.error_message_base(&Method::PUT, path.as_str()),
                &mut response,
            )
            .await?;
        Ok(value)
    }

    /**
     * Idempotently ensures that the given API Disk is attached (or not) as
     * specified.
     */
    pub async fn disk_ensure(
        self: &Arc<Self>,
        disk_id: Uuid,
        initial_runtime: ApiDiskRuntimeState,
        target: ApiDiskStateRequested,
    ) -> Result<ApiDiskRuntimeState, ApiError> {
        let path = format!("/disks/{}", disk_id);
        let body = Body::from(
            serde_json::to_string(&DiskEnsureBody {
                initial_runtime,
                target,
            })
            .unwrap(),
        );
        let mut response =
            self.client.request(Method::PUT, path.as_str(), body).await?;
        /* TODO-robustness handle 300-level? */
        assert!(response.status().is_success());
        let value = self
            .client
            .read_json::<ApiDiskRuntimeState>(
                &self.client.error_message_base(&Method::PUT, path.as_str()),
                &mut response,
            )
            .await?;
        Ok(value)
    }
}

/**
 * Exposes additional [`SledAgentClient`] interfaces for use by the test suite
 */
#[async_trait]
pub trait SledAgentTestInterfaces {
    async fn instance_finish_transition(&self, id: Uuid);
    async fn disk_finish_transition(&self, id: Uuid);
}

#[async_trait]
impl SledAgentTestInterfaces for SledAgentClient {
    async fn instance_finish_transition(&self, id: Uuid) {
        let path = format!("/instances/{}/poke", id);
        let body = Body::empty();
        self.client
            .request(Method::POST, path.as_str(), body)
            .await
            .expect("instance_finish_transition() failed unexpectedly");
    }

    async fn disk_finish_transition(&self, id: Uuid) {
        let path = format!("/disks/{}/poke", id);
        let body = Body::empty();
        self.client
            .request(Method::POST, path.as_str(), body)
            .await
            .expect("instance_finish_transition() failed unexpectedly");
    }
}
