use serde::{
    Deserialize,
    Serialize,
};

use diesel::{
    r2d2::{
        ConnectionManager,
        PooledConnection,
    },
    ExpressionMethods,
    QueryDsl,
    Queryable,
    RunQueryDsl,
    SqliteConnection,
};

use crate::{
    errors::AppError,
    routes::{
        self,
        DbPool,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
#[diesel(table_name = permission)]
pub struct Permission {
    pub id: i32,
    pub name: String,
}

impl Permission {
    fn get_conn(
        pool: &DbPool
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, AppError> {
        Ok(routes::get_connection_from_pool(pool))
    }
    
    pub fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<i32>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::permission::dsl::*;
        
        let res = permission.filter(
            id.eq_any(elements)
        ).load::<Self>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}
