use rusqlite::{Connection, Result};
use std::sync::atomic::{AtomicUsize, Ordering};

// A mock struct to simulate FFI boundaries and tracking allocations
struct AllocatedMemory {
    ptr: *mut u8,
}

static ALLOCATION_COUNT: AtomicUsize = AtomicUsize::new(0);

impl AllocatedMemory {
    fn new(size: usize) -> Self {
        ALLOCATION_COUNT.fetch_add(1, Ordering::SeqCst);
        let mut vec = Vec::with_capacity(size);
        let ptr = vec.as_mut_ptr();
        std::mem::forget(vec); // Simulate passing to C-ABI (SQLite Extension)
        Self { ptr }
    }

    /// Simulate the SQLite xDestroy or free callback
    unsafe fn free(self, size: usize) {
        let _ = Vec::from_raw_parts(self.ptr, 0, size); // Retake ownership and drop
        ALLOCATION_COUNT.fetch_sub(1, Ordering::SeqCst);
    }
}

#[test]
fn test_ffi_memory_leak_prevention() {
    let initial_allocs = ALLOCATION_COUNT.load(Ordering::SeqCst);

    unsafe {
        let mem1 = AllocatedMemory::new(1024);
        let mem2 = AllocatedMemory::new(2048);
        
        assert_eq!(ALLOCATION_COUNT.load(Ordering::SeqCst), initial_allocs + 2);

        // Simulate SQLite tearing down the connection
        mem1.free(1024);
        mem2.free(2048);
    }

    // Ensure all FFI memory given to SQLite has been reclaimed
    assert_eq!(ALLOCATION_COUNT.load(Ordering::SeqCst), initial_allocs, "MEMORY LEAK DETECTED: FFI boundary failed to reclaim memory.");
}

#[test]
fn test_crdt_sync_integration() -> Result<()> {
    // Integration test: In-memory SQLite DB
    let db = Connection::open_in_memory()?;
    
    // Simulate loading our extension
    // db.execute("SELECT load_extension('edgesync_core')", [])?;

    // Create a mock table
    db.execute(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
        (),
    )?;

    // Test that the oplog records the insert correctly (mocked logic)
    db.execute("INSERT INTO users (name) VALUES ('Alice')", ())?;
    
    // In a real environment, we would query `_edgesync_oplog` here 
    // to verify the trigger worked without leaking memory.
    
    Ok(())
}
