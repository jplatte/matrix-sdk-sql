//! SQL State Storage for matrix-sdk
//!
//! ## Usage
//!
//! ```rust,ignore
//!
//! let sql_pool: Arc<sqlx::Pool<DB>> = /* ... */;
//! // Create the state store, applying migrations if necessary
//! let state_store = StateStore::new(&sql_pool).await?;
//!
//! ```
//!
//! After that you can pass it into your client builder as follows:
//!
//! ```rust,ignore
//! let store_config = StoreConfig::new().state_store(Box::new(state_store));
//!
//! let client_builder = Client::builder()
//!                     /* ... */
//!                      .store_config(store_config)
//! ```
//!
//! ## About Trait bounds
//!
//! The list of trait bounds may seem daunting, however every implementation of [`SupportedDatabase`] matches the trait bounds specified.

use std::sync::Arc;

use anyhow::Result;

pub mod helpers;
pub use helpers::SupportedDatabase;
use sqlx::{migrate::Migrate, Database, Pool};
mod statestore;

/// SQL State Storage for matrix-sdk
#[derive(Clone, Debug)]
#[allow(single_use_lifetimes)]
pub struct StateStore<DB: SupportedDatabase> {
    /// The database connection
    db: Arc<Pool<DB>>,
}

#[allow(single_use_lifetimes)]
impl<DB: SupportedDatabase> StateStore<DB> {
    /// Create a new State Store and automtaically performs migrations
    ///
    /// # Errors
    /// This function will return an error if the migration cannot be applied
    pub async fn new(db: &Arc<Pool<DB>>) -> Result<Self>
    where
        <DB as Database>::Connection: Migrate,
    {
        let db = Arc::clone(db);
        let migrator = DB::get_migrator();
        migrator.run(&*db).await?;
        Ok(Self { db })
    }
}
