use std::sync::Mutex;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

use mini_ap::{
    api::{
        actor::{create_test, get_actor, get_instance_actor},
        inbox::{inspect_inbox, private_inbox, shared_inbox, Inbox},
        object::{get_object, get_object_create},
        outbox::create_post,
        webfinger::webfinger,
    },
    config::get_config,
    db::{conn::Conn, postgres::pg_conn::PgConn, InstanceActor},
};
use tokio_postgres::NoTls;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // env::set_var("RUST_BACKTRACE", "1");

    let config = match get_config() {
        Ok(x) => x,
        Err(x) => {
            eprintln!("{:#?}", x);
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

    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });

    {
        let conn = Box::new(PgConn { db: pool.clone() }) as Box<dyn Conn>;
        if let Err(x) = conn.init().await {
            eprintln!("{}", x);
            return Ok(());
        }
        InstanceActor::init_instance_actor(&conn).await;
    }

    println!(
        "starting server at http://{}:{}",
        &config.bind_address, &config.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(
                Box::new(PgConn { db: pool.clone() }) as Box<dyn Conn>
            ))
            .app_data(inbox.clone())
            .app_data(Data::new(config.to_owned()))
            .service(hello)
            .service(webfinger)
            .service(create_post)
            .service(get_object)
            .service(get_object_create)
            .service(get_actor)
            .service(get_instance_actor)
            .service(create_test)
            .service(private_inbox)
            .service(shared_inbox)
            .service(inspect_inbox)
    })
    .bind((bind, port))?
    .run()
    .await
}
