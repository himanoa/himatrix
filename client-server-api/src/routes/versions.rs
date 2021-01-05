use tide::prelude::*;
use tide::Request;

pub async fn versions_endpoint<T>(mut _req: Request<T>) -> tide::Result {
    Ok(json!({
        "versions": vec!["r0.6.1"],
        "unstable_feature": json!({})
    })
    .into())
}
