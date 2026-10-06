use crate::id::{Id, marker::SkuMarker};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Nameplate {
    /// Path to the nameplate asset.
    pub asset: String,
    /// The label of this nameplate.
    pub label: String,
    /// Background color of the nameplate.
    pub palette: NameplatePalette,
    /// ID of the nameplate SKU.
    pub sku_id: Id<SkuMarker>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NameplatePalette {
    Berry,
    BubbleGum,
    Clover,
    Cobalt,
    Crimson,
    Forest,
    Lemon,
    Sky,
    Teal,
    Violet,
    White,
}
