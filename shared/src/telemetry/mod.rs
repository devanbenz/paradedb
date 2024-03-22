mod bgworker;
mod controller;
mod event;
mod postgres;
mod posthog;

use self::event::TelemetryEvent;
pub use bgworker::{setup_telemetry_background_worker, ParadeExtension};
use pgrx::spi::SpiError;
use std::{env::VarError, path::PathBuf, str::Utf8Error};
use thiserror::Error;

pub trait TelemetryStore {
    type Error;

    fn get_connection(
        &self,
    ) -> Result<Box<dyn TelemetryConnection<Error = Self::Error>>, Self::Error>;
}

pub trait TelemetryConnection {
    type Error;

    fn send(&self, uuid: &str, event: &TelemetryEvent) -> Result<(), Self::Error>;
}

pub trait DirectoryStore {
    type Error;

    fn root_path(&self) -> Result<PathBuf, Self::Error>;
    fn extension_path(&self) -> Result<PathBuf, Self::Error>;
    fn extension_size(&self) -> Result<u64, Self::Error>;
    fn extension_uuid(&self) -> Result<String, Self::Error>;
    fn extension_uuid_path(&self) -> Result<PathBuf, Self::Error>;
}

pub trait TermPoll {
    fn term_poll(&self) -> bool;
}

pub trait TelemetryConfigStore {
    fn telemetry_enabled(&self) -> Result<bool, TelemetryError>;
    fn extension_name(&self) -> Result<String, TelemetryError>;
    fn telemetry_api_key(&self) -> Result<String, TelemetryError>;
    fn telemetry_host_url(&self) -> Result<String, TelemetryError>;
    fn root_data_directory(&self) -> Result<PathBuf, TelemetryError>;
}

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("could not de-toast extension name for telemetry: {0}")]
    DetoastExtensionName(#[source] Utf8Error),
    #[error("could not check telemetry file for handled status: {0}")]
    HandledCheck(#[source] std::io::Error),
    #[error("could not read PGDATA variable for telemetry director: {0}")]
    NoPgData(#[source] VarError),
    #[error("could not read telemetry config: {0}")]
    ConfigEnv(#[source] envy::Error),
    #[error("could not send telemetry request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("could not write uuid file: {0}")]
    WriteUuid(#[source] std::io::Error),
    #[error("could not read uuid file: {0}")]
    ReadUuid(#[source] std::io::Error),
    #[error("could not parse uuid file: {0}")]
    ParseUuid(#[source] uuid::Error),
    #[error("missing posthog api key")]
    PosthogApiKey,
    #[error("missing posthog api key")]
    PosthogHost,
    #[error("unknown extension name: {0}")]
    UnknownExtension(String),
    #[error("error checking telemetry enabled guc config: {0}")]
    EnabledCheck(#[source] SpiError),
    #[error("could not lock spi connection in telemetry config")]
    SpiConnectLock(String),
}
