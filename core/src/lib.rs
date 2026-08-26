// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

pub mod crdt;
pub mod p2p;
pub mod plugin;
pub mod vfs;

use sqlite_loadable::Result;
use sqlite_loadable::prelude::*;

/// Entry point for the SQLite loadable extension
#[sqlite_entrypoint]
pub fn sqlite3_edgesync_init(db: *mut sqlite3) -> Result<()> {
    // Register the custom HLC (Hybrid Logical Clock) generator function
    crdt::register_functions(db)?;
    
    // In a full implementation, we would also initialize the VFS layer
    // and potentially spawn the P2P background thread here if not managed by host App.
    Ok(())
}
