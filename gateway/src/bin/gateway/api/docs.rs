/*
    Appellation: docs <api>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::routes::index;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        index::landing
    ),
    components(
        schemas()
    ),
    modifiers(),
    tags(
        (name = "Index", description = "Gateway Index")
    )
)]
pub struct ApiDoc;

// struct SecurityAddon;

// impl Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         if let Some(components) = openapi.components.as_mut() {
//             components.add_security_scheme(
//                 "api_key",
//                 SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
//             )
//         }
//     }
// }
