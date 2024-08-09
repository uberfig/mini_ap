use std::ops::DerefMut;

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

use mini_ap::{config::Config, db::pg_conn::PgConn};
use refinery::Migration;
use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");

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

    let db_config = deadpool_postgres::Config {
        user: Some(config.pg_user.clone()),
        password: Some(config.pg_password.clone()),
        host: Some(config.pg_host.clone()),
        dbname: Some(config.pg_dbname.clone()),

        ..Default::default()
    };

    let pool = db_config.create_pool(None, NoTls).unwrap();

    let mut conn = pool.get().await.expect("could not get conn for migrations");
    let client = conn.deref_mut().deref_mut();
    let report = embedded::migrations::runner().run_async(client).await;
    match report {
        Ok(x) => {
            println!("migrations sucessful");
            // println!("{:?}", x);
            if x.applied_migrations().is_empty() {
                println!("no migrations applied")
            } else {
                println!("applied migrations: ");
                for migration in x.applied_migrations() {
                    match migration.applied_on() {
                        Some(x) => println!(" - {} applied {}", migration.name(), x),
                        None => println!(" - {} applied N/A", migration.name()),
                    }
                }
            }
        }
        Err(x) => {
            println!("{:?}", x);
            return Ok(());
        }
    }

    println!(
        "starting server at http://{}:{}",
        &config.bind_address, &config.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(PgConn { db: pool.clone() }))
            .app_data(Data::new(config.to_owned()))
            .service(hello)
    })
    .bind((bind, port))?
    .run()
    .await
}
