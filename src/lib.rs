use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    stack: Vec<String>,
}
enum Msg {
    Push(String),
}

impl Model {
    fn render_item(&self, value: &str) -> Html {
        html! { <li>{ value }</li> }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            stack: vec![],
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::Push(v) => self.stack.push(v),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <button onclick = self.link.callback(|_| Msg::Push("Value A".to_string()))>{ "Push Value A" }</button>
                <input type = "text" onkeypress = self.link.batch_callback(|e : KeyboardEvent| if e.key() == "Enter" { vec![Msg::Push("Blah blah".to_string())] } else { vec![] }) />
                <ul>
                    { for self.stack.iter().map(|val| self.render_item(val)) }
                </ul>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
