use actix_web::{
    body::BoxBody,
    dev::{
        ServiceRequest,
        ServiceResponse,
    },
    middleware::Next,
    web::Data,
    Error,
    HttpResponse,
};

use crate::{
    models::{
        security::Role,
        user_role::UserRole,
    },
    routes::DbPool,
};

use super::{
    errors::AppError,
    validate_token,
};

enum PermissionNeeded {
    Create,
    Update,
    Delete,
    AssignRole,
}

impl PermissionNeeded {
    fn to_string(&self) -> &'static str {
        match self {
            PermissionNeeded::Create => "CREATE",
            PermissionNeeded::Delete => "DELETE",
            PermissionNeeded::Update => "UPDATE",
            PermissionNeeded::AssignRole => "ASSIGN_ROLE",
        }
    }
}

fn get_token(
    req: &ServiceRequest,
) -> Result<String, AppError> {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                return Ok(String::from(token));
            }
        }
    }
    Err(AppError::InvalidToken)
}

fn get_pool(
    req: &ServiceRequest
) -> &DbPool {
    req.app_data::<Data<DbPool>>()
        .expect("Failed to get pool")
        .as_ref()
}

fn validate_permission(
    req: &ServiceRequest,
    p: PermissionNeeded,
) -> Result<(), AppError> {
    // Get token from request.
    let token = get_token(&req)?; 

    // Vaidate token.
    let claim = validate_token(&token)?;
    
    // Get user role.
    let pool = get_pool(req);

    match UserRole::get_user_role(pool, claim.sub) {
        Ok(role_id) => {
            let permissions = Role::get_permissions(pool, role_id)?;
    
            if !permissions.into_iter().any(|permission| p.to_string() == permission) {
                return Err(AppError::Forbidden);
            }

            Ok(())
        }
        Err(_) => Err(AppError::RoleNeeded)
    }
}

// TODO: For now I'll keep it like this, improve in the future.
pub async fn restrict_create(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    match validate_permission(&req, PermissionNeeded::Create) {
        Ok(_) => next.call(req).await,
        Err(e) => {
            let (req, _) = req.into_parts();
            let response: HttpResponse<BoxBody> = HttpResponse::from_error(e).map_into_boxed_body();
            Ok(ServiceResponse::new(req, response))
        }
    }
}

pub async fn restrict_update(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    match validate_permission(&req, PermissionNeeded::Update) {
        Ok(_) => next.call(req).await,
        Err(e) => {
            let (req, _) = req.into_parts();
            let response: HttpResponse<BoxBody> = HttpResponse::from_error(e).map_into_boxed_body();
            Ok(ServiceResponse::new(req, response))
        }
    }
}

pub async fn restrict_delete(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    match validate_permission(&req, PermissionNeeded::Delete) {
        Ok(_) => next.call(req).await,
        Err(e) => {
            println!("Error: {e}");
            let (req, _) = req.into_parts();
            let response: HttpResponse<BoxBody> = HttpResponse::from_error(e).map_into_boxed_body();
            Ok(ServiceResponse::new(req, response))
        }
    }
}

pub async fn restrict_assign_role(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    match validate_permission(&req, PermissionNeeded::AssignRole) {
        Ok(_) => next.call(req).await,
        Err(e) => {
            let (req, _) = req.into_parts();
            let response: HttpResponse<BoxBody> = HttpResponse::from_error(e).map_into_boxed_body();
            Ok(ServiceResponse::new(req, response))
        }
    }
}
