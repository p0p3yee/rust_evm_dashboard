
#[derive(Clone)]
pub enum FormFieldState {
    DefaultState,
    ErrorState(String),
    ValidState,
}

impl FormFieldState {
    pub fn is_valid(&self) -> bool {
        match self {
            FormFieldState::ValidState => true,
            _ => false,
        }
    }

    pub fn is_default(&self) -> bool {
        match self {
            FormFieldState::DefaultState => true,
            _ => false,
        }
    }

    pub fn is_error(&self) -> bool {
        match self {
            FormFieldState::ErrorState(_) => true,
            _ => false,
        }
    }

    pub fn get_error_msg(&self) -> String {
        match self {
            FormFieldState::ErrorState(s) => s.to_string(),
            _ => "".to_string(),
        }
    }
}

impl Default for FormFieldState {
    fn default() -> Self {
        Self::DefaultState
    }
}