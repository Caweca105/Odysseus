use async_graphql::{Context, EmptySubscription, InputObject, Object, Result, Schema};
use sqlx::PgPool;

// Define the User type with the fields you requested.
#[derive(sqlx::FromRow, async_graphql::SimpleObject)]
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

// Input object for creating a user.
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

// Input object for updating a user.
#[derive(InputObject)]
pub struct UpdateUser {
    // Here we use the `id` field to identify the user.
    pub id: i32,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub comment: Option<String>,
    pub location: Option<String>,
    pub name: Option<String>,
    pub preferences: Option<String>,
}

// Query object providing read operations.
pub struct Query;

#[Object]
impl Query {
    // Fetch a single user by id.
    async fn user(&self, ctx: &Context<'_>, id: i32) -> Result<Option<User>> {
        let pool = ctx.data::<PgPool>()?;
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(user)
    }

    // List all users.
    async fn users(&self, ctx: &Context<'_>) -> Result<Vec<User>> {
        let pool = ctx.data::<PgPool>()?;
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(pool)
            .await?;
        Ok(users)
    }
}

// Mutation object providing create, update, and delete operations.
pub struct Mutation;

#[Object]
impl Mutation {
    // Create a new user.
    async fn create_user(&self, ctx: &Context<'_>, input: NewUser) -> async_graphql::Result<User> {
        let pool = ctx.data::<sqlx::PgPool>()?;
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

    // Update an existing user.
    async fn update_user(&self, ctx: &Context<'_>, input: UpdateUser) -> Result<User> {
        let pool = ctx.data::<PgPool>()?;
        // Retrieve the current user to merge fields.
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

    // Delete a user by id.
    async fn delete_user(&self, ctx: &Context<'_>, id: i32) -> Result<bool> {
        let pool = ctx.data::<PgPool>()?;
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

// In your main function, build the schema with Query and Mutation.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_cors::Cors;
    use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
    use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
    use sqlx::postgres::PgPoolOptions;

    // Initialize logging and load environment variables.
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

    // Build the GraphQL schema including Query and Mutation.
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(pool.clone())
        .finish();

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
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(schema.clone()))
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
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
