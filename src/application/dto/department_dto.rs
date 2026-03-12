use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateDepartmentDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateDepartmentDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 characters"
    ))]
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DepartmentResponse {
    pub uuid: String,
    pub name: String,
}
