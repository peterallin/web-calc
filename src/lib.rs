#![recursion_limit = "1024"]
use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::utils::document;
use yew::web_sys::HtmlElement;

mod calculator;
use calculator::{Calculator, StackValue};

struct Model {
    link: ComponentLink<Self>,
    calculator: Calculator,
    entry: String,
    entry_ref: NodeRef,
}
#[derive(Debug, Clone)]
enum Msg {
    Drop,
    Push,
    ImpliedPush,
    Add,
    SetEntry(String),
}

lazy_static! {
    static ref OPERATORS: std::collections::HashMap<String, Msg> =
        vec![("+".to_string(), Msg::Add),].into_iter().collect();
}

impl Model {
    fn render_stack_value(&self, value: &StackValue) -> Html {
        html! { <li>{ value.as_string() }</li> }
    }
}

fn entry_onkeypress(e: KeyboardEvent) -> Vec<Msg> {
    if let Some(op) = OPERATORS.get(&e.key()).cloned() {
        vec![Msg::ImpliedPush, op]
    } else if e.key() == "Enter" {
        vec![Msg::Push]
    } else {
        vec![]
    }
}

fn entry_oninput(input: InputData) -> Msg {
    if OPERATORS.get(&input.value).is_none() {
        Msg::SetEntry(input.value)
    } else {
        Msg::SetEntry("".to_string())
    }
}

impl Model {
    fn push(&mut self) {
        // TODO: Show parse errors in UI?
        if let Ok(value) = self.entry.parse() {
            self.calculator.push(value);
            self.entry = "".into();
        }
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
                if self.entry == "" {
                    self.calculator.dup();
                } else {
                    self.push();
                }
            }
            Msg::ImpliedPush => {
                self.push();
            }
            Msg::Add => {
                self.calculator.add();
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
        let plus = "+";
        html! {
            <div>
                <input type = "text" id="entry"
                       value = &self.entry
                       onkeypress = self.link.batch_callback(entry_onkeypress)
                       oninput = self.link.callback(entry_oninput)
                       ref = self.entry_ref.clone()
                />
                <button onclick = self.link.callback(|_| Msg::Drop)>{drop}</button>
                <button onclick = self.link.callback(|_| Msg::Add)>{plus}</button>
                <ul>
                    { for self.calculator.stack_iter().map(|val| self.render_stack_value(val)) }
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
