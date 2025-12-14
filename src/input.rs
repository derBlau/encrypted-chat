//! This modules handles the logic for capturing user input
//! All the functionality is organised under the enum Input,
//! which contains no fields. The enum is used simply to
//! make the import of the functionality more organised.

/// this enum contains no fields and is only used to organise
/// the functionality under a category that makes importing
/// cleaner
pub enum Input {
    Valid(String),
    Invalid(String),
    Exit,
}

impl Input {
    /// this function is used to capture input from the user,
    /// which will then be sent to the server. The captured input
    /// is returned as [`Option<Input>`]. If the user is empty or
    /// is empty after trimming, the function returns [`None`], else
    /// it will return some input variant that must be further processed.
    /// The point behind the [`Input::Exit`] variant is having some user control
    /// as to when to disconnect from the server.
    /// Do notice that [`Option<String>`] is only available as a parameter for
    /// testing purposes only. The function should only be called with [`None`].
    pub async fn get_input(input: Option<String>) -> Option<Input> {
        let input: String = input.unwrap_or_else(|| {
            let mut buffer = String::with_capacity(1024);
            print!(">>> ");
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("Failed to read user input");

            buffer.trim().to_string()
        });

        if input.trim().is_empty() {
            return None;
        };

        let input_size = input.len();

        if input_size >= 1025 {
            let res = Self::Invalid("Input exceeds char limit ({input_size}/1024)".to_string());
            return Some(res);
        };

        match input.as_str() {
            "!Ex" => Some(Input::Exit),

            _ => {
                let message = Input::Valid(input);

                Some(message)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn positive_test_returns_valid_string_variant() {
        match Input::get_input(Some("test".to_string()))
            .await
            .expect("Expected some input")
        {
            Input::Valid(_) => {}
            _ => panic!("Expected Input::Valid variant"),
        }
    }

    #[tokio::test]
    async fn positive_case_returns_none_when_entering_an_empty_string() {
        assert!(Input::get_input(Some(" ".to_string())).await.is_none());
    }

    #[tokio::test]
    async fn positive_case_returns_none_when_entering_blank_space_only() {
        assert!(Input::get_input(Some("  ".to_string())).await.is_none());
    }
    #[tokio::test]
    async fn positive_case_returns_invalid_string_variant_when_input_is_too_long() {
        let random_filler_char = "a";
        let too_long_input = random_filler_char.to_string().repeat(2000);

        let test = async |input: Option<String>| match Input::get_input(input)
            .await
            .expect("Expected some input")
        {
            Input::Invalid(_) => {}
            _ => panic!("Expected Input::Invalid variant"),
        };

        test(Some(too_long_input)).await;
    }

    #[tokio::test]
    async fn positive_case_returns_exit_variant() {
        match Input::get_input(Some("!Ex".to_string()))
            .await
            .expect("Expected some input")
        {
            Input::Exit => {}
            _ => panic!("Expected Input::Exit variant"),
        }
    }
}
