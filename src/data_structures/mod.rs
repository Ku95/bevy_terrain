//! This module contains the two fundamental data structures of the terrain:
//! the [`Quadtree`](quadtree::Quadtree) and the [`NodeAtlas`](node_atlas::NodeAtlas).
//!
//! # Explanation
//! Each terrain possesses one [`NodeAtlas`](node_atlas::NodeAtlas), which can be configured
//! to store any [`AtlasAttachment`] required (eg. height, density, albedo, splat, edc.)
//! These attachments can vary in resolution and texture format.
//!
//! To decide which nodes should be currently loaded you can create multiple
//! [`Quadtree`](quadtree::Quadtree) views that correspond to one node atlas.
//! These quadtrees request and release nodes from the node atlas based on their quality
//! setting (`load_distance`).
//! Additionally they are also used to access the best loaded data at any position.
//!
//! Both the node atlas and the quadtrees also have a corresponding GPU representation,
//! which can be used to access the terrain data in shaders.

use bevy::{prelude::*, render::render_resource::*};

pub mod gpu_node_atlas;
pub mod gpu_quadtree;
pub mod node_atlas;
pub mod quadtree;

// Todo: may be swap to u64 for giant terrains
// Todo: consider 3 bit face data, for cube sphere
/// A globally unique identifier of a node.
/// lod |  x |  y
///   4 | 14 | 14
pub type NodeId = u32;
pub const INVALID_NODE_ID: NodeId = NodeId::MAX;

/// Identifier of a node (and its attachments) inside the node atlas.
pub type AtlasIndex = u16;
pub const INVALID_ATLAS_INDEX: AtlasIndex = AtlasIndex::MAX;

pub const INVALID_LOD: u16 = u16::MAX;

/// Identifier of an attachment inside the node atlas.
pub type AttachmentIndex = usize;

/// The global coordinate of a node.
pub struct NodeCoordinate {
    /// The lod of the node, where 0 is the highest level of detail with the smallest size
    /// and highest resolution
    pub lod: u32,
    /// The x position of the node in node sizes.
    pub x: u32,
    /// The y position of the node in node sizes.
    pub y: u32,
}

impl From<NodeId> for NodeCoordinate {
    /// Determines the coordinate of the node based on its id.
    #[inline]
    fn from(id: NodeId) -> Self {
        Self {
            lod: (id >> 28) & 0xF,
            x: (id >> 14) & 0x3FFF,
            y: id & 0x3FFF,
        }
    }
}

/// Calculates the node identifier from the node coordinate.
#[inline]
pub fn calc_node_id(lod: u32, x: u32, y: u32) -> NodeId {
    (lod & 0xF) << 28 | (x & 0x3FFF) << 14 | y & 0x3FFF
}

/// Configures an attachment of a [`NodeAtlas`](node_atlas::NodeAtlas).
#[derive(Clone)]
pub struct AtlasAttachment {
    /// The handle of the attachment array texture.
    pub(crate) handle: Handle<Image>,
    /// The name of the attachment.
    pub(crate) name: &'static str,
    /// The none overlapping texture size in pixels.
    pub(crate) texture_size: u32,
    /// The overlapping border size around the texture, used to prevent sampling artifacts.
    pub(crate) border_size: u32,
    /// The format of the attachment.
    pub(crate) format: TextureFormat,
}
