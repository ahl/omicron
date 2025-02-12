// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use bytes::{BufMut, Bytes, BytesMut};
use schemars::JsonSchema;
use serde::Deserialize;
use slog::Logger;

/// Path parameters for Silo requests
#[derive(Clone, Debug, Deserialize, JsonSchema)]
#[allow(dead_code)]
pub(crate) struct ArtifactId {
    /// The artifact's name.
    pub(crate) name: String,

    /// The version of the artifact.
    pub(crate) version: String,
}

/// The artifact store, used to store artifacts in-memory or via reference.
#[derive(Debug)]
pub(crate) struct ArtifactStore {
    log: Logger,
    // TODO: implement this
}

impl ArtifactStore {
    pub(crate) fn new(log: &Logger) -> Self {
        let log = log.new(slog::o!("component" => "artifact store"));
        Self { log }
    }

    pub(crate) fn get_artifact(&self, id: &ArtifactId) -> Option<Bytes> {
        // This is a test artifact name used by the installinator.
        if id.name == "__installinator-test" {
            // For testing, the version is the size of the artifact.
            let size: usize = id
                .version
                .parse()
                .map_err(|err| {
                    slog::warn!(
                        self.log,
                        "for installinator-test, version should be a usize indicating the size but found {}: {err}",
                        id.version
                    );
                })
                .ok()?;
            let mut bytes = BytesMut::with_capacity(size as usize);
            bytes.put_bytes(0, size);
            return Some(bytes.freeze());
        }

        slog::debug!(self.log, "Artifact requested (this is a stub implementation which always 404s): {:?}", id);
        // TODO: implement this
        None
    }
}
