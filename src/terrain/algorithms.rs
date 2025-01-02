use bevy::prelude::*;
pub mod node;
pub mod wilsons;
pub mod wilsons_bounded;

#[derive(Resource, PartialEq)]
pub enum TerrainAlgorithm {
    Wilsons,
    WilsonsBounded,
}
