use console::Style;
use dotenv::dotenv;
use std::env;

use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;

use tokio_postgres::tls::NoTls;
use tonic::transport::Server;

pub mod user_crud {
    tonic::include_proto!("user_crud");
}
use user_crud::user_crud_server::UserCrudServer;

mod service;
use crate::service::MyUserCrud;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:55555".parse().unwrap();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pg_mgr = PostgresConnectionManager::new_from_stringlike(&database_url, NoTls).unwrap();
    let pool = Pool::builder().build(pg_mgr).await.unwrap();
    let user_crud: MyUserCrud = MyUserCrud { pool: pool.clone() };
    let green = Style::new().green();

    println!("\nListening at {}", green.apply_to(addr));

    Server::builder()
        .add_service(UserCrudServer::new(user_crud))
        .serve(addr)
        .await?;

    Ok(())
}
