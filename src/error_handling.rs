#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct CompilerResult<T> {
    pub result: Option<T>,
    pub errors: Vec<CompilerError>,
}

impl<T> CompilerResult<T> {
    pub fn new(result: Option<T>, errors: Vec<CompilerError>) -> Self {
        CompilerResult { result, errors }
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}