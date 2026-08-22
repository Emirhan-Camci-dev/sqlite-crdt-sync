// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

/// The core plugin trait allows external crates to inject logic into the sync lifecycle
/// without requiring conditional `isPro` compilation flags.
pub trait SyncPlugin: Send + Sync {
    /// Called before an Oplog entry is gossiped to a peer.
    /// If it returns `false`, the entry is blocked from being sent.
    /// Pro modules can use this to implement RBAC filtering.
    fn before_gossip(&self, peer_id: &str, table_name: &str, delta_json: &str) -> bool {
        true // Default allow
    }

    /// Called before applying an incoming remote Oplog entry to the local DB.
    fn before_apply(&self, peer_id: &str, table_name: &str, delta_json: &str) -> bool {
        true
    }

    /// Provides custom encryption key management.
    /// In core, this might just return a fixed key. In Pro, it interfaces with HSM.
    fn get_encryption_key(&self) -> Option<[u8; 32]> {
        None // Fallback to default mechanism
    }
}

pub struct PluginRegistry {
    plugins: Vec<Box<dyn SyncPlugin>>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn SyncPlugin>) {
        self.plugins.push(plugin);
    }
    
    pub fn can_gossip(&self, peer_id: &str, table_name: &str, delta: &str) -> bool {
        self.plugins.iter().all(|p| p.before_gossip(peer_id, table_name, delta))
    }
}
