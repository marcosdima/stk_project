use std::fmt::Debug;

use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::Deserialize;

use crate::errors::AppError;
use crate::routes::DbPool;

pub trait Model: Sized + Debug + PartialEq + for<'a> Deserialize<'a> {
    type NewT;
    type UpdateT;

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

    fn get_all(
        pool: &DbPool,
    ) -> Result<Vec<Self>,AppError>;

    fn get_by_id(
        pool: &DbPool,
        element_id: String
    ) -> Result<Self,AppError>;

    fn delete(
        pool: &DbPool,
        element_id: String
    ) -> Result<usize,AppError>;

    fn update(
        pool: &DbPool,
        data: Self::UpdateT
    ) -> Result<(),AppError>;
}