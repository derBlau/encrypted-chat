//! This modules handles the logic for capturing user input
//! All the functionality is organised under the struct Input,
//! which contains no fields. The struct is used simply to
//! make the import of the functionality more organised.

/// this module contains no fields and is only used to organise
/// the functionality under a category that makes importing
/// cleaner
pub struct Input {}

impl Input {
    /// this function is used to capture input from the user,
    /// which will then be sent to the server. The captured input
    /// is returned as [`Option<String>`] and is expected to be
    /// used to either proceed with the sending of the message
    /// or terminate the connection with the server.
    pub fn get_input() -> Option<String> {
        todo!()
    }
}
