use std::sync::Mutex;

use activity_playground::{
    activitystream_objects::{
        core_types::ActivityStream,
        object::Object,
    },
    api::{
        activities::{get_activity, get_object},
        actor::{create_test, get_actor, post_test},
        inbox::{inspect_inbox, private_inbox, shared_inbox, Inbox},
        webfinger::webfinger,
    },
    cache_and_fetch::Cache,
    config::Config,
    db::conn::DbConn,
    protocol::instance_actor::InstanceActor,
};
use actix_web::{
    // error::ErrorBadRequest,
    get,
    web::{self, Data},
    App,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};
// use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, query};
use url::Url;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/@{preferred_username}")]
async fn get_profile_page(/*conn: Data<DbConn>, */ path: web::Path<String>) -> Result<String> {
    // let val = sqlx::query!(
    //     "INSERT INTO internal_users (password, preferredUsername) VALUES ($1, $2)",
    //     "hi".to_string(),
    //     "hi".to_string()
    // )
    // .execute(&conn.db)
    // .await;

    let preferred_username = path.into_inner();
    Ok(preferred_username)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let test = ActivityStream::Object(Object { context: Context::Array(vec!["hi".to_string(), "hello".to_string()]), id: "hi".to_string(), name: "hi".to_string() });
    // println!("{}", serde_json::to_string(&test).unwrap());

    let test = Object::new(Url::parse("https://test.com/hi").unwrap())
        .name(Some("hello".to_string()))
        .to_activitystream();
    let test = serde_json::to_string_pretty(&test).unwrap();
    println!("{test}");

    let deserialized: ActivityStream = serde_json::from_str(
        r#"{
  "@context": [
    "test1",
    "test2"
  ],
  "type": "Object",
  "id": "https://test.com/hi",
  "name": "hello"
}"#,)
    .unwrap();

    // let deserialized: ActivityStream = serde_json::from_str(
    //     r#"{"type":"Object","@context":["https://context1.com","https://context2.com"],"id":"hi","name":"hi"}"#,
    // )
    // .unwrap();
    dbg!(&deserialized);

    let test = serde_json::to_string_pretty(&deserialized).unwrap();

    println!("{test}");
    //----------------config file settings----------------

    let settings = config::Config::builder()
        // Add in `./Settings.toml`
        .add_source(config::File::with_name("gater_config"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::default())
        .build();

    let settings = match settings {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{:#?}", x);
            return Ok(());
        }
    };

    let config = match settings.try_deserialize::<Config>() {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{:#?}", error);
            return Ok(());
        }
    };

    let bind = config.bind_address.clone();
    let port = config.port;

    //-------------database ------------------

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Error building a connection pool");

    //-------------init instance actor----------------

    let instance_actor = query!(r#"SELECT * FROM instance_actor LIMIT 1"#,)
        .fetch_optional(&pool)
        .await;

    let instance_actor = match instance_actor.unwrap() {
        Some(x) => InstanceActor::new(
            openssl::rsa::Rsa::private_key_from_pem(x.private_key.as_bytes()).unwrap(),
            x.public_key_pem,
            &config.instance_domain,
        ),
        None => {
            let rsa = openssl::rsa::Rsa::generate(2048).unwrap();
            let private_key = String::from_utf8(rsa.private_key_to_pem().unwrap()).unwrap();
            let public = String::from_utf8(rsa.public_key_to_pem().unwrap()).unwrap();

            let val = query!(
                r#"INSERT INTO instance_actor 
                    (private_key, public_key_pem)
                VALUES
                    ($1, $2)
                "#,
                &private_key,
                &public,
            )
            .execute(&pool)
            .await;

            val.unwrap();
            InstanceActor::new(
                openssl::rsa::Rsa::private_key_from_pem(private_key.as_bytes()).unwrap(),
                public,
                &config.instance_domain,
            )
        }
    };

    //-------------------------------------------------

    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    let cache = Data::new(Cache::new(instance_actor, config.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(DbConn { db: pool.clone() }))
            .app_data(Data::new(config.to_owned()))
            .app_data(inbox.clone())
            .app_data(cache.clone())
            .service(hello)
            .service(webfinger)
            .service(get_actor)
            .service(get_profile_page)
            .service(get_activity)
            .service(get_object)
            .service(create_test)
            .service(post_test)
            .service(shared_inbox)
            .service(private_inbox)
            .service(inspect_inbox)
    })
    .bind((bind, port))?
    .run()
    .await
}
