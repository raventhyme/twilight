use crate::id::{
    Id,
    marker::{ChannelMarker, GuildMarker},
};
use serde::{Deserialize, Serialize};

/// Sent when the voice channel start time changes.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct VoiceChannelStartTimeUpdate {
    /// Guild ID.
    pub guild_id: Id<GuildMarker>,
    /// Channel ID.
    pub id: Id<ChannelMarker>,
    /// Unix timestamp (in seconds) of when the voice session started.
    pub voice_start_time: Option<u64>,
}
