/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! # Firefox Accounts Client
//!
//! The fxa-client component lets applications integrate with the
//! [Firefox Accounts](https://mozilla.github.io/ecosystem-platform/docs/features/firefox-accounts/fxa-overview)
//! identity service. The shape of a typical integration would look
//! something like:
//!
//! * Out-of-band, register your application with the Firefox Accounts service,
//!   providing an OAuth `redirect_uri` controlled by your application and
//!   obtaining an OAuth `client_id`.
//!
//! * On application startup, create a [`FirefoxAccount`] object to represent the
//!   signed-in state of the application.
//!     * On first startup, a new [`FirefoxAccount`] can be created by calling
//!       [`FirefoxAccount::new`] and passing the application's `client_id`.
//!     * For subsequent startups the object can be persisted using the
//!       [`to_json`](FirefoxAccount::to_json) method and re-created by
//!       calling [`FirefoxAccount::from_json`].
//!
//! * When the user wants to sign in to your application, direct them through
//!   a web-based OAuth flow using [`begin_oauth_flow`](FirefoxAccount::begin_oauth_flow)
//!   or [`begin_pairing_flow`](FirefoxAccount::begin_pairing_flow); when they return
//!   to your registered `redirect_uri`, pass the resulting authorization state back to
//!   [`complete_oauth_flow`](FirefoxAccount::complete_oauth_flow) to sign them in.
//!
//! * Display information about the signed-in user by using the data from
//!   [`get_profile`](FirefoxAccount::get_profile).
//!
//! * Access account-related services on behalf of the user by obtaining OAuth
//!   access tokens via [`get_access_token`](FirefoxAccount::get_access_token).
//!
//! * If the user opts to sign out of the application, calling [`disconnect`](FirefoxAccount::disconnect)
//!   and then discarding any persisted account data.

mod account;
mod auth;
mod device;
mod error;
mod migration;
mod profile;
mod storage;
mod telemetry;
mod token;

pub use auth::{AuthorizationInfo, MetricsParams};
pub use device::{
    AccountEvent, AttachedClient, Device, DeviceCapability, DevicePushSubscription,
    IncomingDeviceCommand, SendTabPayload, TabHistoryEntry,
};
pub use error::FxaError;
pub use migration::{FxAMigrationResult, MigrationState};
pub use profile::Profile;
pub use token::{AccessTokenInfo, AuthorizationParameters, ScopedKey};

// All the implementation details live in this "internal" module.
// Aspirationally, I'd like to make it private, so that the public API of the crate
// is entirely the same as the API exposed to consumers via UniFFI. That's not
// possible right now because some of our tests/example use features that we do
// not currently expose to consumers. But we should figure out how to expose them!
pub mod internal;

/// Object representing the signed-in state of an application.
///
/// The `FirefoxAccount` object is the main interface provided by this crate.
/// It represents the signed-in state of an application that may be connected to
/// user's Firefox Account, and provides methods for inspecting the state of the
/// account and accessing other services on behalf of the user.
///
pub struct FirefoxAccount {
    // For now, we serialize all access on a single `Mutex` for thread safety across
    // the FFI. We should make the locking more granular in future.
    internal: std::sync::Mutex<internal::FirefoxAccount>,
}

impl FirefoxAccount {
    /// Create a new [`FirefoxAccount`] instance, not connected to any account.
    ///
    /// **💾 This method alters the persisted account state.**
    ///
    /// This method constructs as new [`FirefoxAccount`] instance configured to connect
    /// the application to a user's account.
    ///
    /// # Arguments
    ///
    ///   - `content_url` - the URL of the Firefox Accounts server to use
    ///       - For example, use `https://accounts.firefox.com` for the main
    ///         Mozilla-hosted service.
    ///   - `client_id` - the registered OAuth client id of the application.
    ///   - `redirect_uri` - the registered OAuth redirect URI of the application.
    ///   - `token_server_url_override`: optionally, URL for the user's Sync Tokenserver.
    ///        - This can be used to support users who self-host their sync data.
    ///          If `None` then it will default to the Mozilla-hosted Sync server.
    ///
    pub fn new(
        content_url: &str,
        client_id: &str,
        redirect_uri: &str,
        token_server_url_override: &Option<String>,
    ) -> FirefoxAccount {
        FirefoxAccount {
            internal: std::sync::Mutex::new(internal::FirefoxAccount::new(
                content_url,
                client_id,
                redirect_uri,
                token_server_url_override.as_deref(),
            )),
        }
    }
}

uniffi::include_scaffolding!("fxa_client");
