use utoipa::openapi::{Components, Info, Object, OneOf, OpenApi, Paths, Ref, RefOr, Schema};

pub mod hello_world;
pub mod auth;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    hello_world::hello_world_controller,
    auth::register_controller,
))]
pub struct ApiDoc;

impl ApiDoc {
    pub fn new() -> OpenApi {
        let mut api = <ApiDoc as utoipa::OpenApi>::openapi();
        let mut componenets = Components::new();
        componenets.schemas.insert("Uuid".to_string(), utoipa::openapi::RefOr::T(Schema::Object(Object::new())));
        api.components = Some(componenets);


        api.merge(<hello_world::HelloWorldApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<auth::AuthApiDoc as utoipa::OpenApi>::openapi());
        api.info = Info::new("Hotel API", "0.1.0");
        
        
        api
    }
}