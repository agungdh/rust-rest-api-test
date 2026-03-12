pub mod errors;
pub mod repository;

pub use errors::AppError;
pub use repository::{EmployeeRepository, InMemoryEmployeeRepository};
