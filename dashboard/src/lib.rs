use core::Challenge;
extern crate stdweb;
#[macro_use]
extern crate yew;

mod logger;
mod markdown;

pub use crate::logger::InMemoryDrain;
pub use crate::markdown::render_markdown;

const INTRO: &str = "
Solution to random challenges taken from the 
[r/dailyprogrammer](https://www.reddit.com/r/dailyprogrammer/) subreddit written
in Rust and compiled to *Web Assembly*.

Inspecting a challenge will automatically execute it in the browser and write
all logged output to the *Challenge Output* section.
";

use core::slog::{self, Drain, Logger};
use easy_370::Easy370;
use easy_371::Easy371;
use easy_374::Easy374;
use easy_375::Easy375;
use intermediate_374::Intermediate374;
use intermediate_375::Intermediate375;
use std::sync::Arc;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode, VTag};

pub fn all_challenges() -> Vec<Box<dyn Challenge>> {
    vec![
        Box::new(Easy370::default()),
        Box::new(Easy371::default()),
        Box::new(Easy374::default()),
        Box::new(Easy375::default()),
        Box::new(Intermediate374::default()),
        Box::new(Intermediate375::default()),
    ]
}

#[derive(Debug, Clone)]
pub enum Msg {}

pub struct Model {
    challenges: Vec<ChallengePanel>,
}

impl Model {
    pub fn challenge_cards(&self) -> VNode<Self> {
        let mut list = VList::new();

        for panel in &self.challenges {
            list.add_child(panel.view());
        }

        list.into()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            challenges: all_challenges().into_iter().map(From::from).collect(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="container mb-md-2",>
                <div class="row",>
                    <h1>{"Michael's Daily Programmer Challenges"}</h1>
                </div>

                <div class="row my-md-1",>{markdown::render_markdown(INTRO)}</div>

                <div class="accordian", id="accordian",>
                    {self.challenge_cards()}
                </div>
            </div>
        }
    }
}

/// A panel for displaying a information about a challenge or running it.
pub struct ChallengePanel {
    inner: Box<dyn Challenge>,
}

impl ChallengePanel {
    pub fn evaluate_output(&self) -> String {
        let drain = Arc::new(InMemoryDrain::default());
        let logger = Logger::root(Arc::clone(&drain).fuse(), slog::o!());

        if let Err(e) = self.inner.execute(&logger) {
            slog::error!(logger, "Evaluation failed"; "error" => e.to_string());
        }

        drain.to_string()
    }
}

impl<T: Component> Renderable<T> for ChallengePanel {
    fn view(&self) -> Html<T> {
        let info = self.inner.info();
        let header =
            format!("{} (#{}, {})", info.title, info.number, info.difficulty);

        let header_id = format!("header-{}-{}", info.difficulty, info.number);
        let target_id = format!("body-{}-{}", info.difficulty, info.number);

        let header = yew::html! {
            <div class="card-header",
                 id={header_id},
                 data-toggle="collapse",
                 data-target={format!("#{}", target_id)},>
                 <div class="row",>
                    <div class="col",>
                        <h3>{header}</h3>
                    </div>
                    <div class="col-md-1",>
                        <a class="btn btn-outline-danger btn-sm reddit-logo-btn ml-md-auto",
                        href={info.link.to_string()},>
                            <img src="reddit.png", width="24",></img>
                        </a>
                    </div>
                 </div>
            </div>
        };

        let body = yew::html! {
            <div class="collapse", id={target_id}, data-parent="#accordian",>
                <div class="card-body",>
                {render_markdown(&info.description)}
                <hr></hr>
                <h3>{"Challenge Output"}</h3>
                <pre><code>{{self.evaluate_output()}}</code></pre>
                </div>
            </div>
        };

        let mut card = VTag::new("div");
        card.add_class("card");
        card.add_child(header);
        card.add_child(body);
        card.into()
    }
}

impl From<Box<dyn Challenge>> for ChallengePanel {
    fn from(other: Box<dyn Challenge>) -> ChallengePanel {
        ChallengePanel { inner: other }
    }
}
