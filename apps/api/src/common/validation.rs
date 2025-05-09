use validator::{Validate, ValidationError as ValidatorError, ValidationErrors};
use serde::Deserialize;
use crate::common::errors::{AppError, ValidationError};
use regex::Regex;
use std::collections::HashMap;

pub const MIN_PASSWORD_LENGTH: usize = 8;
pub const MAX_PASSWORD_LENGTH: usize = 72;
pub const MAX_NAME_LENGTH: usize = 100;
pub const MAX_TITLE_LENGTH: usize = 200;
pub const MAX_DESCRIPTION_LENGTH: usize = 1000;

pub fn validate_password(password: &str) -> Result<(), ValidatorError> {
    if password.len() < MIN_PASSWORD_LENGTH {
        return Err(ValidatorError::new("password_too_short"));
    }
    if password.len() > MAX_PASSWORD_LENGTH {
        return Err(ValidatorError::new("password_too_long"));
    }
    
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_digit(10));
    let has_special = password.chars().any(|c| !c.is_alphanumeric());

    if !has_uppercase || !has_lowercase || !has_digit || !has_special {
        return Err(ValidatorError::new("password_requirements_not_met"));
    }

    Ok(())
}

pub fn validate_phone_number(phone: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap();
    if !re.is_match(phone) {
        return Err(ValidatorError::new("invalid_phone_number"));
    }
    Ok(())
}

pub fn validate_url(url: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^https?://[\w\-\.]+(:\d+)?(/[\w\-\./%\+@&#=\(\)]*)?$").unwrap();
    if !re.is_match(url) {
        return Err(ValidatorError::new("invalid_url"));
    }
    Ok(())
}

pub fn validate_date_format(date: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    if !re.is_match(date) {
        return Err(ValidatorError::new("invalid_date_format"));
    }
    Ok(())
}

pub fn validate_hex_color(color: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$").unwrap();
    if !re.is_match(color) {
        return Err(ValidatorError::new("invalid_hex_color"));
    }
    Ok(())
}

pub fn validate_ip_address(ip: &str) -> Result<(), ValidatorError> {
    let ipv4_re = Regex::new(r"^(\d{1,3}\.){3}\d{1,3}$").unwrap();
    let ipv6_re = Regex::new(r"^([0-9a-fA-F]{1,4}:){7}[0-9a-fA-F]{1,4}$").unwrap();
    
    if !ipv4_re.is_match(ip) && !ipv6_re.is_match(ip) {
        return Err(ValidatorError::new("invalid_ip_address"));
    }
    Ok(())
}

pub fn validate_slug(slug: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^[a-z0-9]+(?:-[a-z0-9]+)*$").unwrap();
    if !re.is_match(slug) {
        return Err(ValidatorError::new("invalid_slug"));
    }
    Ok(())
}

pub fn validate_file_size(size: u64, max_size: u64) -> Result<(), ValidatorError> {
    if size > max_size {
        return Err(ValidatorError::new("file_too_large"));
    }
    Ok(())
}

pub fn validate_file_extension(filename: &str, allowed: &[&str]) -> Result<(), ValidatorError> {
    if let Some(ext) = filename.split('.').last() {
        if !allowed.contains(&ext.to_lowercase().as_str()) {
            return Err(ValidatorError::new("invalid_file_extension"));
        }
        Ok(())
    } else {
        Err(ValidatorError::new("no_file_extension"))
    }
}

pub fn validate_email(email: &str) -> Result<(), ValidatorError> {
    let re = Regex::new(r"^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
    
    if email.len() > 254 {
        return Err(ValidatorError::new("email_too_long"));
    }
    
    if !re.is_match(email) {
        return Err(ValidatorError::new("invalid_email"));
    }
    
    if !email.contains('@') || !email.contains('.') {
        return Err(ValidatorError::new("invalid_email"));
    }
    
    Ok(())
}

pub trait ValidateExt {
    fn validate_into_app_error(&self) -> Result<(), AppError>;
}

impl<T: Validate> ValidateExt for T {
    fn validate_into_app_error(&self) -> Result<(), AppError> {
        match self.validate() {
            Ok(()) => Ok(()),
            Err(errors) => {
                let validation_errors = convert_validation_errors(errors);
                Err(AppError::validation_error(validation_errors))
            }
        }
    }
}

fn convert_validation_errors(errors: ValidationErrors) -> Vec<ValidationError> {
    errors
        .field_errors()
        .iter()
        .flat_map(|(field, error_vec)| {
            error_vec.iter().map(|error| ValidationError {
                field: field.to_string(),
                code: error.code.to_string(),
                message: error.message
                    .as_ref()
                    .map(|m| m.to_string())
                    .unwrap_or_else(|| get_error_message(&error.code, *field)),
            })
        })
        .collect()
}

fn get_error_message(code: &str, field: &str) -> String {
    match code {
        "required" => format!("The field '{}' is required", field),
        "password_too_short" => format!("Password must be at least {} characters", MIN_PASSWORD_LENGTH),
        "password_too_long" => format!("Password must not exceed {} characters", MAX_PASSWORD_LENGTH),
        "password_requirements_not_met" => "Password must contain uppercase, lowercase, number, and special character".to_string(),
        "invalid_phone_number" => "Invalid phone number format".to_string(),
        "invalid_url" => "Invalid URL format".to_string(),
        "invalid_date_format" => "Date must be in YYYY-MM-DD format".to_string(),
        "invalid_hex_color" => "Invalid hex color code".to_string(),
        "invalid_ip_address" => "Invalid IP address".to_string(),
        "invalid_slug" => "Invalid slug format".to_string(),
        "file_too_large" => format!("File size exceeds maximum allowed for field '{}'", field),
        "invalid_file_extension" => "File type not allowed".to_string(),
        "no_file_extension" => "File must have an extension".to_string(),
        "invalid_email" => "Invalid email format".to_string(),
        "email_too_long" => "Email address is too long".to_string(),
        _ => format!("Validation failed for field '{}'", field),
    }
}
