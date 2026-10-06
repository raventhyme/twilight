mod nameplate;

pub use self::nameplate::{Nameplate, NameplatePalette};

use serde::{Deserialize, Serialize};

/// The collectibles a user has, excluding Avatar Decorations and Profile
/// Effects.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Collectibles {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nameplate: Option<Nameplate>,
}
