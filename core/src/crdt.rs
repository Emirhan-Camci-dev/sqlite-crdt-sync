// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

use sqlite_loadable::{api, define_scalar_function, Result};
use sqlite_loadable::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static HLC_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Generates a Hybrid Logical Clock (HLC) string.
/// Format: <physical_time_micros_hex>-<logical_counter_hex>
pub fn edgesync_now(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64;
    
    let counter = HLC_COUNTER.fetch_add(1, Ordering::SeqCst);
    
    // Combining physical time with a counter provides causal ordering 
    // even if physical clocks are identical or slightly skewed.
    let hlc = format!("{:016x}-{:04x}", now, counter);
    
    api::result_text(context, hlc)?;
    Ok(())
}

/// Helper function to return the local node's Peer ID / Site ID
pub fn edgesync_site_id(context: *mut sqlite3_context, _values: &[*mut sqlite3_value]) -> Result<()> {
    // In a real app, this would be loaded from a config table or memory
    api::result_text(context, "local-site-xyz-123")?;
    Ok(())
}

pub fn register_functions(db: *mut sqlite3) -> Result<()> {
    define_scalar_function(
        db, 
        "edgesync_now", 
        0, 
        edgesync_now, 
        FunctionFlags::UTF8
    )?;
    
    define_scalar_function(
        db, 
        "edgesync_site_id", 
        0, 
        edgesync_site_id, 
        FunctionFlags::UTF8 | FunctionFlags::DETERMINISTIC
    )?;
    
    Ok(())
}
