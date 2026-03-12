use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateEmployeeDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,

    #[validate(length(min = 1, message = "Position is required"))]
    pub position: String,

    #[validate(range(min = 1, message = "Salary must be greater than 0"))]
    pub salary: i64,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateEmployeeDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(length(min = 1, message = "Position is required"))]
    pub position: Option<String>,

    #[validate(range(min = 1, message = "Salary must be greater than 0"))]
    pub salary: Option<i64>,
}
