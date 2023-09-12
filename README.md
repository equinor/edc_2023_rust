# EDC 2023 rust

Kjell Wilhelm Kongsvik

kwko@equinor.com


Useful links:
 - https://doc.rust-lang.org
 - https://play.rust-lang.org/?version=stable&mode=debug&edition=2021



## Presentation, Repo, codespaces

<https://github.com/equinor/edc_2023_rust>



### Create a new Hello world project
```sh
cargo new <name_of_project>
cd name_of_project
```

```rust
fn main() {
    println!("Hello, world!");
}
```

```sh
cargo run
```


```sh
cargo build
cargo b
```
```sh
cargo run
cargo r
```
```sh
cargo clippy
```
```sh
ls ./target/debug
```
```sh
cargo b --release
ls ./target/release
```


### Ownership
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
}
```
fix this


    
### Immutable by default
```rust
fn main() {
    let a = 42;
    a = 43;
    println!("{a}");
}
```
fix this



### Testing
https://doc.rust-lang.org/rust-by-example/testing.html


#### Unit testing

```rust
fn add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
```
fix this
```sh
cargo test
```


### Integration testing
https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html



### Error handling

Rust do not use `exceptions`

Return `Option` or `Result`


### Option

`enum`: 
 - `Some<T>`: An element of type `T`
 - `None`


```rust
fn divide(a: i64, b: i64) -> Option<i64> {
    if b == 0 {
        return None;
    } else {
        return Some(a / b);
    }
}

fn main() {
    println!("{:?}", divide(5, 2));
}

```
Refactor this - use `cargo clippy`


### Result

```rust
fn to_number(s: &str) -> i64 {
    s.parse().unwrap() + 1
}

fn main() {
    println!("{:?}", to_number("41"))
}
```
Remove unwrap


### destructuring
https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_structures.html




### impl and traits
Rust do not have classes

Traits are somewhat like interfaces


#### Rectangle

```rust
struct Rect {
    x: i64,
    y: i64,
}

fn main() {
    let r = Rect { x: 1, y: 2 };
    println!("{:?}", r);
}
```
Fix this


#### debug trait
```rust
#[derive(Debug)]
struct Rect {
    x: i64,
    y: i64,
}

fn main() {
    let r = Rect { x: 1, y: 2 };
    println!("{:?}", r);
}
```


#### new - by convention
```rust
impl Rect {
    fn new() -> Self {
        Self { x: 1, y: 1 }
    }
}

fn main() {
    let r = Rect::new();
    println!("{:?}", r);
}
```


#### impl display trait for rect
https://doc.rust-lang.org/rust-by-example/hello/print/print_display.html

```rust
impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}x{}", self.x, self.y)
    }
}
```



#### serde

<https://serde.rs>

```rust
#[derive(Serialize, Deserialize, Debug)]
struct Rect {
    x: i64,
    y: i64,
}
```

```rust
    let r = Rect::new();
    println!("{}", r);

    let serialized = serde_json::to_string(&r).unwrap();

    println!("serialized = {}", serialized);

    let deserialized: Rect = serde_json::from_str(&serialized).unwrap();

    println!("deserialized = {:?}", deserialized);
```



## web frameworks
 - actix
 - rocket
 - axum


### axum
 - From the https://tokio.rs project
 - https://docs.rs/axum/latest/axum


### Axum - Hello world
Install depencies
```sh
cargo add tokio --features macros,rt-multi-thread 
cargo add axum
```


```rust
use axum::{routing::get, Router};

async fn handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```



### Extractors


#### extract path

```rust
async fn user(Path(user_id): Path<String>) -> String {
    user_id
}

// add
// .route("/one/:user_id", get(user));

```


#### extract query

```rust
async fn query(Query(params): Query<HashMap<String, String>>)
 -> Result<String, StatusCode> {
    Ok(params["key"].clone())
}
```
Handle error


#### extract json
```sh
cargo add serde_json
```

```rust
async fn json(Json(payload): Json<serde_json::Value>)
 -> String {
    serde_json::to_string(&payload).unwrap()
}
// .route("/json", post(json));
```
Add error handling
```sh
curl -H "Content-Type: application/json" -X POST \
--data '{"key":"val"}' http://localhost:3000/json
```


#### extract bearer token
```sh
cargo add axum --features headers
```

```rust
async fn bearer(TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>) -> String {
    bearer
}
```
fix this



### State

```rust
#[derive(Clone)]
pub struct AppState {
    pub secret: String,
}

impl FromRef<AppState> for String {
    fn from_ref(state: &AppState) -> Self {
        state.secret.clone()
    }
}

async fn handler(State(state): State<AppState>) -> String {
    String::from_ref(&state)
}

```
```rust
    let state = AppState {
        secret: "secret".to_string(),
    };

    let app = Router::new().route("/", get(handler)).
        with_state(state);

```


### Middleware
https://docs.rs/axum/latest/axum/middleware/index.html

```rust
async fn my_middleware<B>(request: Request<B>, next: Next<B>) -> Response {
    dbg!(request.headers().keys());
    next.run(request).await
}
```
```rust
 .layer(middleware::from_fn(my_middleware));

```



## Security

If your requirements are simple:
 - Consider using `oauth2_proxy`
     - built into `radix`
 - Otherwise consider using `JWT`


### Basic jwt authentication primer
<https://login.microsoftonline.com/3aa4a235-b6e2-48d5-9195-7fcf05b459b0/v2.0/.well-known/openid-configuration>

```js
{
    "jwks_uri":"https://login.microsoftonline.com/3aa4a235-b6e2-48d5-9195-7fcf05b459b0/discovery/v2.0/keys",
    "id_token_signing_alg_values_supported":["rs256"],
    "issuer":"https://login.microsoftonline.com/3aa4a235-b6e2-48d5-9195-7fcf05b459b0/v2.0",
}
```


Never submit a valid token to <https://jwt.io/>

Use <https://jwt.ms/> for valid tokens.


#### get bearer token
```rust
async fn my_middleware<B>(
    TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if bearer.token() != "some_token"{
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}
```


#### verify hs256 bearer token
```sh
cargo add jsonwebtoken
cargo add serde

```

```rust
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
}
```

```rust
async fn my_middleware<B>(
    TypedHeader(Authorization(bearer)): 
        TypedHeader<Authorization<Bearer>>,
    request: Request<B>,
    next: Next<B>,
) -> Response {
    if let Err(_) = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::new(Algorithm::HS256),
    {
        return StatusCode::UNAUTHORIZED.into_response();
    }
    next.run(request).await
}
```

Task: get secret from state



### getting the jwks
 - Write your own
 - https://crates.io/crates/jwt-authorizer
 - https://crates.io/crates/axum-jwks

Who do you trust?


### axum_jwks

We will start with axum-jwks, and potentially modify it.

Inspect code and do this:
```sh
cargo add axum-jwks@0.5.0
```
Or
```
cd ..
git clone git@github.com:cdriehuys/axum-jwks.git
```
Add this to Cargo.toml
```
[dependencies]
axum-jwks = { path="../axum-jwks" }

```

