use async_std::sync::Mutex;
use ring::rand::{self, SecureRandom};
use ring::{digest, pbkdf2};
use std::collections::HashMap;
use std::num::NonZeroU32;
use std::sync::Arc;
use tide::prelude::*;
use tide::Request;

static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
type Credential = [u8; CREDENTIAL_LEN];

#[derive(Clone, Debug)]
struct Database {
    iterations: NonZeroU32,
    salt_component: [u8; 32],
    storage: HashMap<String, Credential>,
}

type DbState = Arc<Mutex<Database>>;

impl Database {
    fn register(&mut self, username: &str, password: &str) -> tide::Result<String> {
        let salt = self.salt(username);
        let mut to_store: Credential = [0u8; CREDENTIAL_LEN];
        pbkdf2::derive(
            PBKDF2_ALG,
            self.iterations,
            &salt,
            password.as_bytes(),
            &mut to_store,
        );
        self.storage.insert(username.into(), to_store);
        Ok("Register successed".into())
    }

    fn verify(&self, username: &str, attempted_password: &str) -> tide::Result<String> {
        match self.storage.get(username) {
            Some(actual_password) => {
                let salt = self.salt(username);
                pbkdf2::verify(
                    PBKDF2_ALG,
                    self.iterations,
                    &salt,
                    attempted_password.as_bytes(),
                    actual_password,
                )
                .map(|_| "Authentication successed".into())
                .map_err(|_| {
                    println!("Authentication failed");
                    tide::Error::from_str(tide::StatusCode::Unauthorized, "Authentication failed")
                })
            }
            None => Err(tide::Error::from_str(
                tide::StatusCode::Unauthorized,
                "User not found",
            )),
        }
    }

    fn salt(&self, username: &str) -> Vec<u8> {
        let mut salt = Vec::with_capacity(self.salt_component.len() + username.as_bytes().len());
        salt.extend(self.salt_component.as_ref());
        salt.extend(username.as_bytes());
        salt
    }
}

#[derive(Debug, Deserialize)]
struct PostBody {
    username: String,
    password: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let rng = rand::SystemRandom::new();
    let mut salt_component = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt_component).unwrap();

    let db = Database {
        iterations: NonZeroU32::new(100_000).unwrap(),
        salt_component,
        storage: HashMap::new(),
    };
    let db = Arc::new(Mutex::new(db));

    tide::log::start();

    let mut app = tide::with_state(db);
    app.at("/register")
        .post(|mut req: Request<DbState>| async move {
            let mut db = req.state().lock_arc().await;
            let PostBody { username, password } = req.body_json().await.unwrap();
            db.register(&username, &password)
        });
    app.at("/auth")
        .post(|mut req: Request<DbState>| async move {
            let db = req.state().lock_arc().await;
            let PostBody { username, password } = req.body_json().await.unwrap();
            db.verify(&username, &password)
        });
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}
