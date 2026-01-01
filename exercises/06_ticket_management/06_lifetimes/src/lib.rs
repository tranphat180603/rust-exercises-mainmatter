use ticket_fields::{TicketDescription, TicketTitle};

// TODO: Implement the `IntoIterator` trait for `&TicketStore` so that the test compiles and passes.
#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

// 1. We declare a lifetime 'a to say "The iterator lives as long as the store"
impl<'a> IntoIterator for &'a TicketStore {
    // 2. We return REFERENCES to tickets, not tickets themselves
    type Item = &'a Ticket;
    
    // 3. We use the slice iterator (which handles &T)
    type IntoIter = std::slice::Iter<'a, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        // 4. We use .iter() to get references
        self.tickets.iter()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }

    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    pub fn iter(&self) -> std::slice::Iter<Ticket> {
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(ticket);

        let ticket = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        };
        store.add_ticket(ticket);

        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = (&store).into_iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
