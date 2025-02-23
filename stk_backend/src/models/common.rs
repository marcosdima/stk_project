use std::fmt::Debug;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::Deserialize;

use crate::errors::AppError;
use crate::routes::DbPool;

pub trait Model: Debug + PartialEq + for<'a> Deserialize<'a> + BasicModel {
    type UpdateT;

    fn get_all(
        pool: &DbPool,
    ) -> Result<Vec<Self>, AppError>;

    fn get_by_id(
        pool: &DbPool,
        element_id: String
    ) -> Result<Self, AppError>;

    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError>;

    fn update(
        pool: &DbPool,
        data: Self::UpdateT
    ) -> Result<(), AppError>;
}

pub trait BasicModel: Sized {
    type NewT;
    type PK;

    fn new(data: Self::NewT) -> Self;

    fn get_conn(
        pool: &DbPool
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, AppError> {
        Ok(pool.get().expect("Couldn't get DB connection from pool"))
    }
    
    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError>;

    fn delete(
        pool: &DbPool,
        element_id: Self::PK
    ) -> Result<usize, AppError>;
}