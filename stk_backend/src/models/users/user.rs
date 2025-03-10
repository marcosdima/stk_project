use serde::{
    Deserialize,
    Serialize
};

use diesel::{
    self,
    prelude::Insertable,
    ExpressionMethods,
    QueryDsl,
    Queryable,
    RunQueryDsl
};

use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        common::Model,
        BasicModel
    },
    routes::DbPool,
    schema::user
};

use super::{
    UserUpdate,
    NewUser,
};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Queryable, Insertable)]
#[diesel(table_name = user)]
pub struct User {
    pub id: String,
    pub name: String,
    pub lastname: String,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(
        name: String,
        lastname: String,
        username: String,
        password_hash: String,
    ) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            name,
            lastname,
            username,
            password_hash,
        }
    }

    pub fn get_by_username(
        pool: &DbPool,
        target: String,
    ) -> Result<Self, AppError> {
        use crate::schema::user::dsl::*;

        if let Ok(found) = user
            .filter(username.eq(target))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("User with username provided does not exist!"))
        }
    }
}

impl Model for User { 
    type UpdateT = UserUpdate;

    fn get_all(
        pool: &DbPool
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::user::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        let res = user.load(conn)?;
        Ok(res)
    }

    fn get_by_id(
        pool: &DbPool,
        element_id: String,
    ) -> Result<Self, AppError> {
        use crate::schema::user::dsl::*;

        if let Ok(found) = user
            .filter(id.eq(element_id))
            .first::<Self>(&mut Self::get_conn(pool)?)
            {
            Ok(found)
        } else {
            Err(AppError::NotFound("User with id provided does not exist!"))
        }
    }

    fn update(
        pool: &DbPool,
        data: Self::UpdateT,
    ) -> Result<(), AppError> {
        use crate::schema::user::dsl::*;

        diesel::update(user.filter(id.eq(&data.id.to_string())))
            .set(&data)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(())
    }
    
    fn get_in_id_array(
        pool: &DbPool,
        elements: Vec<String>
    ) -> Result<Vec<Self>, AppError> {
        use crate::schema::user::dsl::*;
        
        let res = user.filter(
            id.eq_any(elements)
        ).load::<User>(&mut Self::get_conn(pool)?)?;

        Ok(res)
    }
}

impl BasicModel for User {
    type NewT = NewUser;
    type PK = String;
    
    fn new(data: Self::NewT) -> Self {
        User {
            id: Uuid::new_v4().to_string(),
            name: data.name,
            lastname: data.lastname,
            username: data.username,
            password_hash: data.password_hash,
        }
    }

    fn delete(
        pool: &DbPool,
        element_id: Self::PK,
    ) -> Result<usize, AppError> {
        use crate::schema::user::dsl::*;
        let conn = &mut Self::get_conn(pool)?;

        Ok(
            diesel::delete(user.filter(id.eq(element_id)))
                .execute(conn)?
        )
    }
    
    fn create(
        pool: &DbPool,
        data: Self::NewT
    ) -> Result<Self, AppError> {
        let new_object = <Self as BasicModel>::new(data);

        diesel::insert_into(user::table)
            .values(&new_object)
            .execute(&mut Self::get_conn(pool)?)?;

        Ok(new_object)
    }

}
