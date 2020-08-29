mod routes;

extern crate swish_swish;

use swish_swish::*;

use crate::routes::root_handler;

fn swish_swish() -> Swish {
    let mut swish = Swish::new();
    swish.get("/root", root_handler);
    swish.set_address("0.0.0.0");
    swish
}

fn main() {
    swish_swish().bish();
}
