use rig::Embed;
use serde::{Deserialize, Serialize};

#[derive(Embed, Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Document {
    pub id: String,

    #[embed]
    pub message: String,
}
