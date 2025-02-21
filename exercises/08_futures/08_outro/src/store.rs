use std::collections::BTreeMap;

use super::data::Status;
use crate::data::Ticket;
use crate::data::TicketDraft;
use crate::data::TicketId;

pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("create ticket {0} failed")]
    CreateError(TicketDraft),
    #[error("Retrieve ticket {0} failed")]
    RetrieveError(TicketId),
    #[error("Update ticket {0} failed")]
    UpdateError(TicketId),
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn get_all(&self) -> Result<Vec<Ticket>, StoreError> {
        Ok(self.tickets.values().cloned().collect())
    }

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft) -> Result<Ticket, StoreError> {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket_draft_clone = ticket_draft.clone();
        let ticket = Ticket {
            id,
            title: ticket_draft.title,
            description: ticket_draft.description,
            status: Status::ToDo,
        };
        self.tickets
            .insert(id, ticket)
            .ok_or_else(|| StoreError::CreateError(ticket_draft_clone))
    }

    pub fn get(&self, id: TicketId) -> Result<&Ticket, StoreError> {
        self.tickets
            .get(&id)
            .ok_or_else(|| StoreError::RetrieveError(id))
    }

    pub fn update(
        &mut self,
        id: TicketId,
        ticket_draft: TicketDraft,
    ) -> Result<TicketId, StoreError> {
        if let Some(ticket) = self.tickets.get_mut(&id) {
            ticket.title = ticket_draft.title;
            ticket.description = ticket_draft.description;
            return Ok(id);
        }
        Err(StoreError::UpdateError(ticket_draft.id))
    }
}
