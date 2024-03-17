use async_graphql::http::GraphiQLSource;
use async_graphql::*;
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

struct User {
    a: i32,
    b: i32,
    c: i32,
}

struct Query;
#[Object]
impl Query {
    async fn user(&self, a: i32) -> Result<Option<MyObject>> {
        Ok(Some(MyObject { a, b: 10, c: 20 }))
    }

    async fn context<'ctx>(&self, ctx: &Context<'ctx>) -> Result<&'ctx String> {
        ctx.data::<String>()
    }

    async fn test(&self) -> Result<String> {
        Ok("Hello, World!".to_string())
    }
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data("Context data".to_string())
        .finish();

    println!("{}", &schema.sdl());
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));

    println!("Playground: http://localhost:8000");

    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
}
