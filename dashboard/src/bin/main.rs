extern crate dashboard;
extern crate stdweb;
extern crate yew;

use crate::stdweb::web::{self, IParentNode};
use dashboard::Model;
use yew::prelude::App;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();

    let element = web::document().query_selector("#app").unwrap().unwrap();
    app.mount(element);
    yew::run_loop();
}
