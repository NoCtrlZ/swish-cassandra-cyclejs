use swish_swish::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Sample {
    code: u16,
    data: String,
}

pub fn root_handler(_: &Request) -> Box<dyn Body> {
    Box::new(Json(Sample {
        code: 200,
        data: "hello world".to_string(),
    }))
}
