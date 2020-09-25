#![recursion_limit = "1024"]
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;
use yew::web_sys::HtmlElement;

mod calculator;
use calculator::Calculator;

struct Model {
    link: ComponentLink<Self>,
    calculator: Calculator,
    entry: String,
    entry_ref: NodeRef,
}
enum Msg {
    Drop,
    Push,
    SetEntry(String),
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
            calculator: Calculator::new(),
            entry: "".into(),
            entry_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::Drop => {
                let _ = self.calculator.drop();
            }
            Msg::Push => {
                self.calculator.push(self.entry.clone());
                self.entry = "".into();
            }
            Msg::SetEntry(v) => self.entry = v,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let drop = "⏏";
        html! {
            <div>
                <input type = "text" id="entry"
                       value = &self.entry
                       onkeypress = self.link.batch_callback(|e : KeyboardEvent| if e.key() == "Enter" { vec![Msg::Push] } else { vec![] })
                       oninput = self.link.callback(|x : InputData| Msg::SetEntry(x.value))
                       ref = self.entry_ref.clone()
                />
                <button onclick = self.link.callback(|_| Msg::Drop)>{drop}</button>
                <ul>
                    { for self.calculator.stack_iter().map(|val| self.render_item(val)) }
                </ul>
            </div>
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(entry) = self.entry_ref.cast::<HtmlElement>() {
                let _ = entry.focus();
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    let calculator_element = document()
        .query_selector("#calculator")
        .expect("Couldn't find calculator element")
        .expect("Couldn't unwrap calculator element");
    App::<Model>::new().mount(calculator_element);
}
