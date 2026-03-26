use crate::models::types::{ExtractorCard, FlowStep, Layer, ResponseType, RouteRow, Section};

pub fn all_sections() -> Vec<Section> {
    vec![
        Section {
            id: 1,
            slug: "stack".into(),
            title: "The Stack".into(),
            subtitle: "Four layers, one request".into(),
            icon: "⚡".into(),
        },
        Section {
            id: 2,
            slug: "router".into(),
            title: "Router".into(),
            subtitle: "URL patterns to handlers".into(),
            icon: "🔀".into(),
        },
        Section {
            id: 3,
            slug: "handlers".into(),
            title: "Handlers".into(),
            subtitle: "async fn that do the work".into(),
            icon: "⚙️".into(),
        },
        Section {
            id: 4,
            slug: "extractors".into(),
            title: "Extractors".into(),
            subtitle: "Typed data from requests".into(),
            icon: "📦".into(),
        },
        Section {
            id: 5,
            slug: "responses".into(),
            title: "Responses".into(),
            subtitle: "IntoResponse for everything".into(),
            icon: "📤".into(),
        },
        Section {
            id: 6,
            slug: "state".into(),
            title: "State".into(),
            subtitle: "Shared resources across handlers".into(),
            icon: "🗄️".into(),
        },
        Section {
            id: 7,
            slug: "middleware".into(),
            title: "Middleware".into(),
            subtitle: "Tower layers around your app".into(),
            icon: "🛡️".into(),
        },
        Section {
            id: 8,
            slug: "flow".into(),
            title: "Full Flow".into(),
            subtitle: "One request, every layer".into(),
            icon: "🌊".into(),
        },
    ]
}

pub fn all_layers() -> Vec<Layer> {
    vec![
        Layer {
            id: "app",
            name: "Your Application",
            badge: "you write this",
            icon: "🎬",
            description: "Route handlers, templates, database queries, business logic — all your code lives here.",
            detail: "This is where your Rust code lives. You write handler functions, define data models, query the database, and return responses. You import Axum types but you never call Hyper or Tokio directly — the layers below are invisible once set up.",
            color_class: "coral",
        },
        Layer {
            id: "axum",
            name: "Axum",
            badge: "web framework",
            icon: "🔀",
            description: "Router, extractors, handler dispatch, response conversion — the ergonomic API layer.",
            detail: "Axum provides the ergonomic API: Router maps URLs to handlers, extractors parse requests into typed Rust values, and IntoResponse converts your return values to HTTP. Axum is built on Tower and Hyper but gives you a clean, high-level interface.",
            color_class: "blue",
        },
        Layer {
            id: "tower",
            name: "Tower",
            badge: "middleware",
            icon: "🛡️",
            description: "Middleware: auth, logging, rate limiting, timeouts — composable shells around your app.",
            detail: "Tower defines the Service trait: a thing that takes a request and returns a response. Layers stack Services on top of each other. TraceLayer, TimeoutLayer, and CorsLayer are all Tower services. You can write your own auth middleware as a Tower layer too.",
            color_class: "amber",
        },
        Layer {
            id: "hyper",
            name: "Hyper",
            badge: "http engine",
            icon: "🌐",
            description: "HTTP/1.1 and HTTP/2 — parses raw TCP bytes into request objects, serialises responses.",
            detail: "Hyper handles the raw HTTP protocol. It reads bytes from the TCP socket, parses them into method/path/headers/body structures, and serialises your response back to bytes. Axum uses Hyper under the hood — you never interact with it directly.",
            color_class: "teal",
        },
        Layer {
            id: "tokio",
            name: "Tokio",
            badge: "runtime",
            icon: "⚡",
            description: "Async runtime — manages OS threads, event loop, non-blocking I/O. Everything runs on Tokio.",
            detail: "Tokio manages OS threads and the event loop. When a handler awaits a database query, Tokio suspends that task and runs other handlers on the same thread. This is how Axum handles thousands of concurrent connections without thousands of OS threads.",
            color_class: "purple",
        },
    ]
}

pub fn all_routes() -> Vec<RouteRow> {
    vec![
        RouteRow {
            method: "GET",
            pattern: "/movies",
            handler: "list_movies",
            example: "/movies",
            highlight: false,
        },
        RouteRow {
            method: "POST",
            pattern: "/movies",
            handler: "create_movie",
            example: "/movies",
            highlight: false,
        },
        RouteRow {
            method: "GET",
            pattern: "/movies/{id}",
            handler: "get_movie",
            example: "/movies/42",
            highlight: true,
        },
        RouteRow {
            method: "DELETE",
            pattern: "/movies/{id}",
            handler: "delete_movie",
            example: "/movies/42",
            highlight: false,
        },
        RouteRow {
            method: "PUT",
            pattern: "/movies/{id}",
            handler: "update_movie",
            example: "/movies/42",
            highlight: false,
        },
        RouteRow {
            method: "GET",
            pattern: "/people",
            handler: "list_people",
            example: "/people",
            highlight: false,
        },
    ]
}

pub fn all_extractors() -> Vec<ExtractorCard> {
    vec![
        ExtractorCard {
            name: "Path",
            type_sig: "Path(id): Path<i32>",
            icon: "📍",
            description: "Extracts named segments from the URL path. The {id} in your route pattern becomes a typed Rust value — no manual parsing.",
            code: r#"// Route: GET /movies/{id}
async fn get_movie(Path(id): Path<i32>) -> String {
    format!("Movie id: {}", id)  // id is already i32
}

// Multiple path params → tuple
async fn get_role(
    Path((movie_id, role_id)): Path<(i32, i32)>,
) {}"#,
            color: "blue",
        },
        ExtractorCard {
            name: "Query",
            type_sig: "Query(filter): Query<MovieFilter>",
            icon: "🔍",
            description: "Deserialises URL query parameters (?genre=Crime&min_rating=8.0) into a struct. Fields are Option<T> for optional params.",
            code: r#"#[derive(Deserialize)]
struct MovieFilter {
    genre:      Option<String>,
    min_rating: Option<f32>,
}

// GET /movies?genre=Crime&min_rating=8.0
async fn list_movies(
    Query(f): Query<MovieFilter>,
) -> String {
    format!("Genre: {:?}", f.genre)
}"#,
            color: "teal",
        },
        ExtractorCard {
            name: "Json",
            type_sig: "Json(payload): Json<CreateMovieRequest>",
            icon: "📦",
            description: "Deserialises a JSON request body into a typed struct. Returns 422 automatically if the body is malformed.",
            code: r#"#[derive(Deserialize)]
struct CreateMovieRequest {
    title:        String,
    release_year: i32,
}

// POST /movies  {"title":"Pulp Fiction","release_year":1994}
async fn create_movie(
    Json(req): Json<CreateMovieRequest>,
) {
    println!("Creating: {}", req.title);
}"#,
            color: "amber",
        },
        ExtractorCard {
            name: "State",
            type_sig: "State(state): State<AppState>",
            icon: "🗄️",
            description: "Gives the handler a clone of the shared application state — database pool, config, caches. Registered once via .with_state().",
            code: r#"async fn list_movies(
    State(state): State<AppState>,
    Query(filter): Query<MovieFilter>,
) -> Json<Vec<Movie>> {
    // state.db is your PgPool
    let movies = sqlx::query_as!(Movie,
        "SELECT * FROM movie")
        .fetch_all(&state.db)
        .await.unwrap();
    Json(movies)
}"#,
            color: "purple",
        },
    ]
}

pub fn all_response_types() -> Vec<ResponseType> {
    vec![
        ResponseType {
            type_name: "&'static str",
            description: "Plain text string",
            http_info: "200 OK · text/plain",
            color: "blue",
        },
        ResponseType {
            type_name: "Json<T>",
            description: "Serialises any Serialize type to JSON",
            http_info: "200 OK · application/json",
            color: "teal",
        },
        ResponseType {
            type_name: "Html<String>",
            description: "HTML — used with Askama templates",
            http_info: "200 OK · text/html",
            color: "green",
        },
        ResponseType {
            type_name: "StatusCode",
            description: "Any HTTP status, no body",
            http_info: "N code · empty body",
            color: "amber",
        },
        ResponseType {
            type_name: "(StatusCode, T)",
            description: "Combine status + any body via tuple",
            http_info: "N code · any content-type",
            color: "coral",
        },
        ResponseType {
            type_name: "Result<T, AppError>",
            description: "Ok path and Err path, both converted",
            http_info: "200 or error status",
            color: "purple",
        },
    ]
}

pub fn all_flow_steps() -> Vec<FlowStep> {
    vec![
        FlowStep {
            num: 1,
            delay_ms: 0,
            label: "Tokio",
            description: "Async runtime wakes up — bytes arrived on the TCP socket",
            layer: "tokio",
        },
        FlowStep {
            num: 2,
            delay_ms: 100,
            label: "Hyper",
            description: "Raw bytes parsed into HTTP: method, path, headers, body",
            layer: "hyper",
        },
        FlowStep {
            num: 3,
            delay_ms: 200,
            label: "TraceLayer",
            description: "Tower middleware logs the request: method, path, timestamp",
            layer: "tower",
        },
        FlowStep {
            num: 4,
            delay_ms: 300,
            label: "TimeoutLayer",
            description: "Starts a 10-second timer — returns 408 if handler is too slow",
            layer: "tower",
        },
        FlowStep {
            num: 5,
            delay_ms: 400,
            label: "Axum Router",
            description: "GET /movies/42 matches /movies/{id} — dispatches to get_movie",
            layer: "axum",
        },
        FlowStep {
            num: 6,
            delay_ms: 500,
            label: "Extractors",
            description: "Path(42) pulled from URL, State(db) cloned from AppState — passed to handler",
            layer: "axum",
        },
        FlowStep {
            num: 7,
            delay_ms: 600,
            label: "Handler — get_movie",
            description: "Your code runs: queries the database, gets the movie, returns Ok(Json(movie))",
            layer: "handler",
        },
        FlowStep {
            num: 8,
            delay_ms: 700,
            label: "IntoResponse",
            description: "Json(movie) serialised to bytes, status 200, Content-Type: application/json set",
            layer: "axum",
        },
        FlowStep {
            num: 9,
            delay_ms: 800,
            label: "Back through Tower",
            description: "TimeoutLayer cancels timer, TraceLayer logs duration and status code",
            layer: "tower",
        },
        FlowStep {
            num: 10,
            delay_ms: 900,
            label: "Hyper — Tokio — Client",
            description: "Response bytes written to the TCP socket — client receives the response",
            layer: "hyper",
        },
    ]
}
