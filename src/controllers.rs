use utoipa::openapi::{
    security::{HttpBuilder, SecurityScheme},
    Components, Info, Object, OpenApi, Schema,
};

pub mod auth;
pub mod hello_world;

#[derive(utoipa::OpenApi)]
#[openapi(paths(
    hello_world::hello_world_controller,
    auth::register_controller,
    auth::login_controller
))]
pub struct ApiDoc;

impl ApiDoc {
    pub fn new() -> OpenApi {
        let mut api = <ApiDoc as utoipa::OpenApi>::openapi();
        let mut componenets = Components::new();
        componenets.schemas.insert(
            "Uuid".to_string(),
            utoipa::openapi::RefOr::T(Schema::Object(Object::new())),
        );
        componenets.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
        api.components = Some(componenets);

        api.merge(<hello_world::HelloWorldApiDoc as utoipa::OpenApi>::openapi());
        api.merge(<auth::AuthApiDoc as utoipa::OpenApi>::openapi());
        api.info = Info::new("Hotel API", "0.1.0");
        api.info.description = Some("Hotel backend system made in the actix web framework.".to_string());

        api
    }
}
