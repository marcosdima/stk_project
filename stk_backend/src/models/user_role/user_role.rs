use diesel::{
    prelude::{
        Insertable,
        Queryable
    },
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
};

use serde::{
    Serialize,
    Deserialize
};

use crate::{
    errors::AppError,
    routes::DbPool,
    schema::user_role::{
       self,
       role_id,
    }
};
use crate::models::common::BasicModel;

use super::new_user_role::NewUserRole;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = user_role)]
pub struct UserRole {
    user_id: String,
    role_id: i32,
}

impl BasicModel for UserRole {
    type NewT = NewUserRole;
    type PK = (String, i32);

    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(user_role::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::user_role::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let (user_id_, role_id_) = element_id;

        Ok(
            diesel::delete(
                user_role
                    .filter(role_id.eq(role_id_))
                    .filter(user_id.eq(user_id_))
            ).execute(conn)?
        )
    }
    
    fn new(data: Self::NewT) -> Self {
        UserRole {
            role_id: data.role_id,
            user_id: data.user_id,
        }
    } 
}

impl UserRole {
    pub fn get_role_users(
        pool: &DbPool,
        target: i32,
    ) -> Result<Vec<String>, AppError>{
        use crate::schema::user_role::dsl::user_role;

        let conn = &mut Self::get_conn(pool)?;

        let stk_cat_ids: Vec<UserRole> = user_role.filter(role_id.eq(target)).load(conn)?;
        let elements = stk_cat_ids.into_iter().map(|sc| sc.user_id.clone()).collect();
        
        Ok(elements)
    }

    pub fn get_user_role(
        pool: &DbPool,
        target: String,
    ) -> Result<i32, AppError>{
        use crate::schema::user_role::dsl::*;

        let conn = &mut Self::get_conn(pool)?;

        match user_role.filter(user_id.eq(target)).first::<Self>(conn){
            Ok(found) => Ok(found.role_id),
            Err(e) => Err(e.into())
        }
    }
}
