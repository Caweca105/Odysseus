use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "Hello, Rust with GraphQL!"
    }
}

type MySchema = Schema<Query, EmptyMutation, EmptySubscription>;

async fn graphql_handler(schema: web::Data<MySchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging to help debug any errors
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    dotenv::dotenv().ok();
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::resource("/graphql")
                    // Handle GET requests for GraphQL queries
                    .route(web::get().to(graphql_handler))
                    // Handle POST requests for GraphQL queries
                    .route(web::post().to(graphql_handler))
                    // Handle OPTIONS preflight requests
                    .route(
                        web::method(actix_web::http::Method::OPTIONS)
                            .to(|| async { HttpResponse::Ok() }),
                    ),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
