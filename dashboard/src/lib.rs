use core::Challenge;
extern crate stdweb;
#[macro_use]
extern crate yew;

mod logger;
mod markdown;

pub use crate::logger::InMemoryDrain;
pub use crate::markdown::render_markdown;

use core::slog::{self, Drain, Logger};
use easy_371::Easy371;
use easy_374::Easy374;
use easy_375::Easy375;
use intermediate_374::Intermediate374;
use intermediate_375::Intermediate375;
use std::sync::Arc;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

pub fn all_challenges() -> Vec<Box<dyn Challenge>> {
    vec![
        Box::new(Easy371::default()),
        Box::new(Easy374::default()),
        Box::new(Easy375::default()),
        //Box::new(Intermediate374::default()),
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
            <div class="container",>
                <div class="row my-md-2",>
                    <h1>{"Michael's Daily Programmer Challenges"}</h1>
                </div>

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
            format!("{} ({}/{})", info.title, info.number, info.difficulty);

        let header_id = format!("header-{}-{}", info.difficulty, info.number);
        let target_id = format!("body-{}-{}", info.difficulty, info.number);

        yew::html! {
                <div class="card",>
                    <div class="card-header", id={header_id},>
                        <h3>
                            <button class="btn btn-link",
                                    type="button",
                                    data-toggle="collapse",
                                    data-target={format!("#{}", target_id)},>
                            {header}
                            </button>
                        </h3>
                    </div>
                    <div class="collapse", id={target_id}, data-parent="#accordian",>
                        <div class="card-body",>
                        {render_markdown(&info.description)}
                        <hr></hr>
                        <h3>{"Challenge Output"}</h3>
                        <pre><code>{{self.evaluate_output()}}</code></pre>
                        </div>
                    </div>
                </div>
        }
    }
}

impl From<Box<dyn Challenge>> for ChallengePanel {
    fn from(other: Box<dyn Challenge>) -> ChallengePanel {
        ChallengePanel { inner: other }
    }
}
