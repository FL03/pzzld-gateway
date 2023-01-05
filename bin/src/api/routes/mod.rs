/*
   Appellation: routes <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/

pub mod index;
pub mod s3;

pub fn api() -> axum::Router {
    axum::Router::new()
        .merge(index::router())
        .nest("/s3", s3::router())
}
