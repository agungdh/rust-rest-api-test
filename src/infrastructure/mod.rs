pub mod database;
pub mod errors;
pub mod repository;

pub use database::establish_connection;
pub use errors::AppError;
pub use repository::{DepartmentRepository, EmployeeRepository};
