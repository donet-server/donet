//! bevy_donet
//!
//! A Bevy plugin for the Bevy game engine that provides the client-side implementation
//! for Donet. This integrates Donet to the engine by translating network field updates
//! to Bevy ECS operations.

pub mod client_repository;

use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use client_repository::ClientRepositoryPlugin;

/// Plugin group for all Donet Bevy plugins.
pub struct DonetPlugins;

impl PluginGroup for DonetPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>().add(ClientRepositoryPlugin)
    }
}
