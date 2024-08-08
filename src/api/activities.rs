use actix_web::{
    get,
    web::{self},
    HttpResponse, Result,
};

// const TEST_USER: &str = "test";

pub const ACTIVITY: &str = r#"
{
	"@context": "https://www.w3.org/ns/activitystreams",

	"id": "https://place.ivytime.gay/users/test/1/activity",
	"type": "Create",
	"actor": "https://place.ivytime.gay/users/test",

	"object": {
		"id": "https://place.ivytime.gay/users/test/1",
		"type": "Note",
		"published": "2024-06-06T8:24:54Z",
		"attributedTo": "https://place.ivytime.gay/users/test",
		"inReplyTo": "https://mastodon.social/@Gargron/100254678717223630",
		"content": "<p>Hello world from a silly rust implimentation</p>",
		"to": "https://www.w3.org/ns/activitystreams#Public"
	}
}
"#;

const OBJECT: &str = r#"
{
	"@context": "https://www.w3.org/ns/activitystreams",

    "id": "https://place.ivytime.gay/users/test/1",
    "type": "Note",
    "published": "2024-06-06T8:24:54Z",
    "attributedTo": "https://place.ivytime.gay/users/test",
    "inReplyTo": "https://mastodon.social/@Gargron/100254678717223630",
    "content": "<p>Hello world from a silly rust implimentation</p>",
    "to": "https://www.w3.org/ns/activitystreams#Public"
}
"#;

#[get("/users/{preferred_username}/statuses/{id}/activity")]
pub async fn get_activity(path: web::Path<(String, u64)>) -> Result<HttpResponse> {
    let (preferred_username, id) = path.into_inner();
    println!("get activity {preferred_username}, {id}");

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(ACTIVITY))
}

#[get("/users/{preferred_username}/statuses/{id}")]
pub async fn get_object(path: web::Path<(String, u64)>) -> Result<HttpResponse> {
    let (preferred_username, id) = path.into_inner();
    println!("get object {preferred_username}, {id}");

    Ok(HttpResponse::Ok()
        .content_type("application/activity+json; charset=utf-8")
        .body(OBJECT))
}
