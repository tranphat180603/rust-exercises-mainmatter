use core::panic;

// TODO: Use two variants, one for a title error and one for a description error.
//   Each variant should contain a string with the explanation of what went wrong exactly.
//   You'll have to update the implementation of `Ticket::new` as well.
enum TicketNewError {
    TitleError{err_message: String},
    DescriptionError{err_message: String}
}

// TODO: `easy_ticket` should panic when the title is invalid, using the error message
//   stored inside the relevant variant of the `TicketNewError` enum.
//   When the description is invalid, instead, it should use a default description:
//   "Description not provided".
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    let title_clone = title.clone();
    let status_clone = status.clone();

    match Ticket::new(title, description, status){
        Ok(ticket) => ticket,

        Err(err) => {
            match err{
                TicketNewError::TitleError { err_message } => {
                    panic!("{}", err_message);
                },

                TicketNewError::DescriptionError {..} =>{
                    return Ticket {title: title_clone, description: "Description not provided".to_string(), status: status_clone};
                }
            }
        }
    }
    
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError { err_message: ("Title cannot be empty").to_string() })
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError { err_message: ("Title cannot be longer than 50 bytes").to_string() });
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError { err_message: ("Description cannot be empty").to_string() });
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError { err_message: ("Description cannot be longer than 500 bytes").to_string() });
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }

    pub fn set_description(&mut self, new_description: String){
        self.description = new_description;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
