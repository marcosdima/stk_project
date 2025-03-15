use serde::{
    Deserialize,
    Serialize,
};

use diesel::{
    self,
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
    models::security::{
        Permission,
        RolePermission,
    },
    routes::{
        self,
        DbPool,
    },
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable)]
#[diesel(table_name = role)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: String,
}

impl Role {
    fn get_conn(
        pool: &DbPool
    ) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, AppError> {
        Ok(routes::get_connection_from_pool(pool))
    }
    
    pub fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::role::dsl::*;
        let res = role.load(&mut Self::get_conn(pool)?)?;
        Ok(res)
    }

    pub fn get_by_id(
        pool: &DbPool,
        role_id: i32,
    ) -> Result<Self, AppError> {
        use crate::schema::role::dsl::*;

        if let Ok(found) = role
            .filter(id.eq(role_id))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("Role with id provided does not exist!"))
        }
    }

    pub fn get_permissions(
        pool: &DbPool,
        r_id: i32,
    ) -> Result<Vec<String>, AppError> {
        use crate::schema::role_permission::dsl::{ role_permission, role_id };

        let permissions_id: Vec<i32> = role_permission
            .filter(role_id.eq(r_id))
            .load::<RolePermission>(&mut Self::get_conn(pool)?)?
            .into_iter()
            .map(|rp| rp.permission_id)
            .collect();
        
        Ok(
            Permission::get_in_id_array(pool, permissions_id)?
                .into_iter()
                .map(|p| p.name)
                .collect()
        )
    }
}