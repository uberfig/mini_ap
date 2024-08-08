use std::sync::Mutex;


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
use mini_ap::{config::Config, db::conn::DbConn};
use tokio_postgres::NoTls;
use url::Url;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/@{preferred_username}")]
async fn get_profile_page(/*conn: Data<DbConn>, */ path: web::Path<String>) -> Result<String> {

    let preferred_username = path.into_inner();
    Ok(preferred_username)
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

    //-------------init instance actor----------------

    // let instance_actor = init_instance_actpr(
    //     &mut pool.begin().await.expect("failed to establish transaction"),
    //     &config.instance_domain,
    // )
    // .await;

    //-------------------------------------------------

    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    let cache = Data::new(Cache::new(instance_actor, config.clone()));

    //

    // let test = authorized_fetch(
    //     &Url::parse("https://mastodon.social/users/ivy_test").unwrap(),
    //     &cache.instance_actor.item.key_id,
    //     &cache.instance_actor.item.private_key,
    // )
    // .await;
    // dbg!(&test);
    // test.unwrap();
    //

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
            .service(create_test)
            // .service(post_test)
            .service(shared_inbox)
            .service(private_inbox)
            .service(inspect_inbox)
            .service(create_post)
            .service(private_outbox)
            .service(get_object)
            .service(get_instance_actor)
    })
    .bind((bind, port))?
    .run()
    .await
}