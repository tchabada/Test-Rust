use console::Style;
use dotenv::dotenv;
use std::env;

use sqlx::PgPool;

use tonic::transport::Server;

pub mod user_crud {
    tonic::include_proto!("user_crud");
}
use user_crud::user_crud_server::UserCrudServer;

mod service;
use crate::service::MyUserCrud;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::0]:55555".parse().unwrap();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let user_crud: MyUserCrud = MyUserCrud {
        pool: PgPool::new(&database_url).await?.clone(),
    };

    let green = Style::new().green();

    println!("\nListening at {}", green.apply_to(addr));

    Server::builder()
        .add_service(UserCrudServer::new(user_crud))
        .serve(addr)
        .await?;

    Ok(())
}
