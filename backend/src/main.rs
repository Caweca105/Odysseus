use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use async_graphql::{Context, EmptySubscription, InputObject, Object, Result, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

// ------------------ GraphQL Types & Resolvers ------------------

#[derive(sqlx::FromRow, async_graphql::SimpleObject, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub age: Option<i32>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<String>,
}

#[derive(InputObject)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub age: Option<i32>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateUser {
    pub id: i32,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<String>,
}

pub struct Query;

#[Object]
impl Query {
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn create_user(&self, ctx: &Context<'_>, input: NewUser) -> async_graphql::Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, age, comment, location, name, preferences)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             RETURNING *",
        )
        .bind(&input.username)
        .bind(&input.email)
        .bind(input.age)
        .bind(&input.comment)
        .bind(&input.location)
        .bind(&input.name)
        .bind(&input.preferences)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUser) -> Result<User> {
        let pool = ctx.data::<PgPool>()?;
        let current = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(input.id)
            .fetch_one(pool)
            .await?;

        let new_email = input.email.unwrap_or(current.email);
        let new_age = input.age.or(current.age);
        let new_comment = input.comment.or(current.comment);
        let new_location = input.location.or(current.location);
        let new_name = input.name.or(current.name);
        let new_preferences = input.preferences.or(current.preferences);

        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET email = $1, age = $2, comment = $3, location = $4, name = $5, preferences = $6
             WHERE id = $7 RETURNING *",
        )
        .bind(new_email)
        .bind(new_age)
        .bind(new_comment)
        .bind(new_location)
        .bind(new_name)
        .bind(new_preferences)
        .bind(input.id)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

// ------------------ REST API Handlers ------------------

// Handler for listing all users: GET /api/users
async fn get_users(pool: web::Data<PgPool>) -> Result<HttpResponse, actix_web::Error> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(users))
}

// Handler for fetching a single user: GET /api/users/{id}
async fn get_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = user_id.into_inner();
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

// Define a struct to receive new user data from REST (using serde).
#[derive(Deserialize)]
struct NewUserRest {
    username: String,
    email: String,
    age: Option<i32>,
    comment: Option<String>,
    location: Option<String>,
    name: Option<String>,
    preferences: Option<String>,
}

// Handler for creating a user: POST /api/users
async fn create_user(
    pool: web::Data<PgPool>,
    new_user: web::Json<NewUserRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = new_user.into_inner();
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, email, age, comment, location, name, preferences)
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
    )
    .bind(&new_user.username)
    .bind(&new_user.email)
    .bind(new_user.age)
    .bind(&new_user.comment)
    .bind(&new_user.location)
    .bind(&new_user.name)
    .bind(&new_user.preferences)
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Created().json(user))
}

// Define a struct for receiving updates via REST.
#[derive(Deserialize)]
struct UpdateUserRest {
    email: Option<String>,
    age: Option<i32>,
    comment: Option<String>,
    location: Option<String>,
    name: Option<String>,
    preferences: Option<String>,
}

// Handler for updating a user: PUT /api/users/{id}
async fn update_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
    update: web::Json<UpdateUserRest>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let current = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_one(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    let upd = update.into_inner();
    let new_email = upd.email.unwrap_or(current.email);
    let new_age = upd.age.or(current.age);
    let new_comment = upd.comment.or(current.comment);
    let new_location = upd.location.or(current.location);
    let new_name = upd.name.or(current.name);
    let new_preferences = upd.preferences.or(current.preferences);

    let user = sqlx::query_as::<_, User>(
        "UPDATE users SET email = $1, age = $2, comment = $3, location = $4, name = $5, preferences = $6
         WHERE id = $7 RETURNING *",
    )
    .bind(new_email)
    .bind(new_age)
    .bind(new_comment)
    .bind(new_location)
    .bind(new_name)
    .bind(new_preferences)
    .bind(id)
    .fetch_one(pool.get_ref())
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().json(user))
}

// Handler for deleting a user: DELETE /api/users/{id}
async fn delete_user(
    pool: web::Data<PgPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let id = path.into_inner();
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool.get_ref())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;
    if result.rows_affected() > 0 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

// ------------------ Main Function ------------------

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging and environment variables.
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    // Create the PostgreSQL connection pool.
    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create PostgreSQL pool");

    // Build the GraphQL schema.
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool.clone())
        .finish();

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                        actix_web::http::header::ACCEPT,
                    ])
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
            // GraphQL endpoints.
            .service(
                web::resource("/graphql")
                    .route(web::get().to(
                        |schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
                         req: GraphQLRequest| async move {
                            GraphQLResponse::from(schema.execute(req.into_inner()).await)
                        },
                    ))
                    .route(web::post().to(
                        |schema: web::Data<Schema<Query, Mutation, EmptySubscription>>,
                         req: GraphQLRequest| async move {
                            GraphQLResponse::from(schema.execute(req.into_inner()).await)
                        },
                    ))
                    .route(
                        web::method(actix_web::http::Method::OPTIONS)
                            .to(|| async { HttpResponse::Ok() }),
                    ),
            )
            // REST API endpoints under the /api scope.
            .service(
                web::scope("/api")
                    .route("/users", web::get().to(get_users))
                    .route("/users/{id}", web::get().to(get_user))
                    .route("/users", web::post().to(create_user))
                    .route("/users/{id}", web::put().to(update_user))
                    .route("/users/{id}", web::delete().to(delete_user)),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
