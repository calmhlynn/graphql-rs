use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject, http::GraphiQLSource};
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;

#[derive(SimpleObject)]
struct MyObject {
    a: i32,
    b: i32,

    c: i32,
}



struct Query;
#[Object]
impl Query {
    async fn user(&self, a: i32) -> Result<Option<MyObject>, anyhow::Error> {
        Ok(Some(MyObject {
            a,
            b: 10,
            c: 20,
        }))
    }
}



async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
