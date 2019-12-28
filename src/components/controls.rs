use yew::prelude::*;

use crate::Msg as crate_msg;

pub struct Controls {
    title: String,
    exits: [u8; 3],
    onsignal: Option<Callback<crate_msg>>,
}

pub enum Msg {
    ButtonPressed(crate_msg),
}

#[derive(PartialEq, Clone, Properties)]
pub struct Props {
    pub exits: [u8; 3],
    pub onsignal: Option<Callback<crate_msg>>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            exits: [0, 0, 0],
            onsignal: None,
        }
    }
}

impl Component for Controls {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Controls {
            title: "Controls".into(),
            exits: props.exits,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonPressed(msg) => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(msg);
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.exits = props.exits;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<Controls> for Controls {
    fn view(&self) -> Html<Self> {
        let move_button = |target: &u8| {
            let t = *target;
            html! {
                <span class="control-button">
                    <button onclick=|_| Msg::ButtonPressed(crate_msg::SwitchRoom(t)),>
                        {&format!("Move to {}", target)}
                    </button>
                    <button onclick=|_| Msg::ButtonPressed(crate_msg::ShootArrow(t)),>
                        {&format!("Shoot {}", target)}
                    </button>
                </span>
            }
        };
        html! {
            <div class=("container", "container-controls"),>
                <div class="title",>{&self.title}</div>
                <div class="exits",>{ for self.exits.iter().map(move_button) }</div>
            </div>
        }
    }
}
