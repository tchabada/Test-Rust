use uuid::Uuid;

use futures::TryStreamExt;

use sqlx::postgres::PgRow;
use sqlx::types::chrono::NaiveDate;
use sqlx::{PgPool, Row};

use tonic::{Request, Response, Status};

use crate::user_crud::{
    user_crud_server::UserCrud, CreateUserReply, CreateUserRequest, DeleteUserReply, Empty,
    UpdateUserReply, UpdateUserRequest, UserReply, UserRequest, Users,
};

#[derive(Debug)]
pub struct MyUserCrud {
    pub pool: PgPool,
}

#[tonic::async_trait]
impl UserCrud for MyUserCrud {
    async fn get_user(&self, request: Request<UserRequest>) -> Result<Response<UserReply>, Status> {
        println!("Got a request: {:#?}", &request);
        let UserRequest { id } = &request.into_inner();
        let reply = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .try_map(|row: PgRow| {
                Ok(UserReply {
                    id: row.try_get(0).unwrap(),
                    first_name: row.try_get(1).unwrap(),
                    last_name: row.try_get(2).unwrap(),
                    date_of_birth: row.try_get::<NaiveDate, _>(3).unwrap().to_string(),
                })
            })
            .fetch_one(&self.pool)
            .await
            .unwrap();

        Ok(Response::new(reply))
    }

    async fn list_users(&self, request: Request<Empty>) -> Result<Response<Users>, Status> {
        println!("Got a request: {:#?}", &request);

        let v: Vec<UserReply> = sqlx::query("SELECT * FROM users")
            .try_map(|row: PgRow| {
                Ok(UserReply {
                    id: row.try_get(0).unwrap(),
                    first_name: row.try_get(1).unwrap(),
                    last_name: row.try_get(2).unwrap(),
                    date_of_birth: row.try_get::<NaiveDate, _>(3).unwrap().to_string(),
                })
            })
            .fetch(&self.pool)
            .try_collect::<Vec<_>>()
            .await
            .unwrap();

        let reply = Users { users: v };

        Ok(Response::new(reply))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let id = Uuid::new_v4().to_hyphenated().to_string();
        let CreateUserRequest {
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap();
        let number_of_rows_affected = sqlx::query(
            "INSERT INTO users (id, first_name, last_name, date_of_birth) VALUES ($1, $2, $3, $4)",
        )
        .bind(&id)
        .bind(&first_name)
        .bind(&last_name)
        .bind(&serialize_date_of_birth)
        .execute(&self.pool)
        .await
        .unwrap();
        let reply = if number_of_rows_affected == 0 {
            CreateUserReply {
                message: format!("Fail to create user with id {}.", &id),
            }
        } else {
            CreateUserReply {
                message: format!("Create {} user with id {}.", &number_of_rows_affected, &id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn update_user(
        &self,
        request: Request<UpdateUserRequest>,
    ) -> Result<Response<UpdateUserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let UpdateUserRequest {
            id,
            first_name,
            last_name,
            date_of_birth,
        } = &request.into_inner();
        let serialize_date_of_birth = NaiveDate::parse_from_str(date_of_birth, "%Y-%m-%d").unwrap();
        let number_of_rows_affected = sqlx::query(
            "UPDATE users SET first_name = $2, last_name = $3, date_of_birth = $4 WHERE id = $1",
        )
        .bind(&id)
        .bind(&first_name)
        .bind(&last_name)
        .bind(&serialize_date_of_birth)
        .execute(&self.pool)
        .await
        .unwrap();
        let reply = if number_of_rows_affected == 0 {
            UpdateUserReply {
                message: format!("Fail to update the user with id {}.", id),
            }
        } else {
            UpdateUserReply {
                message: format!("Update {} user with id {}", &number_of_rows_affected, &id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_user(
        &self,
        request: Request<UserRequest>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let UserRequest { id } = &request.into_inner();
        let number_of_rows_affected = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(&id)
            .execute(&self.pool)
            .await
            .unwrap();
        let reply = if number_of_rows_affected == 0 {
            DeleteUserReply {
                message: format!("Fail to delete the user with id {}.", id),
            }
        } else {
            DeleteUserReply {
                message: format!("Remove the user with id {}.", id),
            }
        };

        Ok(Response::new(reply))
    }

    async fn delete_users(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<DeleteUserReply>, Status> {
        println!("Got a request: {:#?}", &request);

        let number_of_rows_affected = sqlx::query("DELETE FROM users")
            .execute(&self.pool)
            .await
            .unwrap();
        let reply = DeleteUserReply {
            message: format!(
                "Remove {} user data from the database.",
                number_of_rows_affected
            ),
        };

        Ok(Response::new(reply))
    }
}
