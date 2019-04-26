use core::Challenge;
extern crate stdweb;
#[macro_use]
extern crate yew;

mod markdown;

pub use crate::markdown::render_markdown;

use easy_374::Easy374;
use easy_375::Easy375;
use intermediate_374::Intermediate374;
use intermediate_375::Intermediate375;
use yew::prelude::*;
use yew::virtual_dom::{VList, VNode};

#[derive(Debug, Clone)]
pub enum Msg {}

pub struct Model {
    challenges: Vec<Box<dyn Challenge>>,
}

impl Model {
    pub fn challenge_cards(&self) -> VNode<Self> {
        let mut list = VList::new();

        for challenge in &self.challenges {
            let info = challenge.info();
            let header =
                format!("{} ({}/{})", info.title, info.number, info.difficulty);
            let card = yew::html! {
                <div class="card",>
                    <h3 class="card-title",>{header}</h3>
                    <div class="card-body",>{render_markdown(&info.description)}</div>
                </div>
            };
            list.add_child(card.into());
        }

        list.into()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            challenges: vec![
                Box::new(Easy374::default()),
                Box::new(Easy375::default()),
                Box::new(Intermediate374::default()),
                Box::new(Intermediate375::default()),
            ],
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
                <div class="row",>
                    <h1>{"Michael's Daily Programmer Challenges"}</h1>
                </div>
                <div class="row",>
                {self.challenge_cards()}
                </div>
            </div>
        }
    }
}

pub struct ChallengePanel {
    inner: Box<dyn Challenge>,
}
