use async_graphql::http::GraphiQLSource;
use async_graphql::*;
use async_graphql_axum::GraphQL;
use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

struct User {
    id: ID,
    name: String,
}
#[Object]
impl User {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn name(&self) -> &String {
        &self.name
    }
}

struct Query;

#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: ID) -> User {
        ctx.data::<String>().unwrap();
        User {
            id,
            name: "John Doe".to_string(),
        }
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn mut_user(&self, id: ID) -> User {
        User {
            id,
            name: "John Doe".to_string(),
        }
    }
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data("Hello World!".to_string())
        .finish();

    let app = Router::new().route("/graphql", get(graphiql).post_service(GraphQL::new(schema)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running at http://{}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

