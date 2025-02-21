mod description;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub use self::description::Description;

mod id;
pub use self::id::TicketId;

mod title;
pub use self::title::Title;

mod status;
pub use self::status::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: TicketId,
    pub title: Title,
    pub description: Description,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketDraft {
    pub id: TicketId,
    pub title: Title,
    pub description: Description,
}

impl Display for TicketDraft {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}", self)
    }
}
