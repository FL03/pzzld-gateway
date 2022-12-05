/*
    Appellation: docs <api>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use super::routes;
// use scsys::prelude::Message;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::index::landing,
        routes::index::gateway_info,
        routes::index::s3_info
    ),
    components(
        schemas(routes::s3::BucketParams)
    ),
    modifiers(),
    tags(
        (name = "Gateway", description = "A high-preformance API gateway, primarily targeting decentralized systems")
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
