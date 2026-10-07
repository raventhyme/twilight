//! Models used when setting the channel positions over HTTP.

use serde::{Deserialize, Serialize};

use crate::{
    channel::ChannelFlags,
    id::{Id, marker::ChannelMarker},
};

/// Used to update the position of channels over HTTP.
///
/// ## Note:
///
/// The fields with `Option<Option<T>>` will be `null` if they have
/// the form `Some(None)`, `None` will be skipped.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Position {
    /// Flags of the channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<Option<ChannelFlags>>,
    /// Channel ID.
    pub id: Id<ChannelMarker>,
    /// Syncs the permission overwrites with the new parent, if moving
    /// to a new category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_permissions: Option<Option<bool>>,
    /// The new parent ID for the channel that is moved
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<Option<Id<ChannelMarker>>>,
    /// Sorting position of the channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Option<u64>>,
}

impl From<(Id<ChannelMarker>, u64)> for Position {
    fn from((id, position): (Id<ChannelMarker>, u64)) -> Self {
        Self {
            flags: None,
            id,
            lock_permissions: None,
            parent_id: None,
            position: Some(Some(position)),
        }
    }
}
