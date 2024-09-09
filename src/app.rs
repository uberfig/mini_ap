use std::sync::Mutex;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};

use crate::{
    api::{
        actor::{create_test, get_actor, get_instance_actor},
        inbox::{inspect_inbox, private_inbox, shared_inbox, Inbox},
        object::{get_object, get_object_create},
        outbox::create_post,
        webfinger::webfinger,
    },
    config::Config,
    db::utility::instance_actor::InstanceActor,
};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn start_application(config: Config) -> std::io::Result<()> {
    //init the instance actor
    {
        let conn = config.create_conn();
        if let Err(x) = conn.init().await {
            eprintln!("{}", x);
            return Ok(());
        }
        InstanceActor::init_instance_actor(&*conn).await;
    }

    let bind = config.bind_address.clone();
    let port = config.port;
    let inbox = Data::new(Inbox {
        inbox: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(config.create_conn()))
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
