use actix_web::{
    route,
    web::{self, Data},
    HttpResponse, HttpServer, Responder,
};
use juniper::http::GraphQLRequest;
use schema::create_schema;
use std::{io::Result, sync::Arc};

use crate::schema::{DatabaseContext, Schema};

pub mod mutation;
pub mod query;
pub mod schema;

#[actix_web::main]
async fn main() -> Result<()> {
    let schema = Arc::new(create_schema());
    let database = Arc::new(DatabaseContext::new());
    let app = move || {
        actix_web::App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::from(database.clone()))
            .service(graphql)
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await?;

    Ok(())
}

#[route("/graphql", method = "POST")]
async fn graphql(
    schema: web::Data<Schema>,
    data: web::Json<GraphQLRequest>,
    context: web::Data<DatabaseContext>,
) -> impl Responder {
    let response = data.execute(&schema, &context).await;
    HttpResponse::Ok().json(response)
}
