/*
    Appellation: s3 <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub mod buckets;

pub fn router() -> axum::Router {
    axum::Router::new().nest("/buckets", buckets::router())
}
