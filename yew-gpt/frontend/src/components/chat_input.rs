use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
//use gloo::console::log;

#[derive(Properties, PartialEq, Clone)]
pub struct Props{
    pub onsubmit: Callback<()>,
    pub onchange: Callback<String>,
    pub input: String,
}

#[styled_component(ChatInput)]
pub fn chat_input(props: &Props) -> Html {
    let onsubmit = props.onsubmit.clone();
    let handle_submit = Callback::from(move |e: FocusEvent| {
        e.prevent_default();
        toggle_disable();
        onsubmit.emit(());
    });

    let onchange = props.onchange.clone();
    let handle_change = Callback::from(move |event: Event| {
        event.prevent_default();
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        onchange.emit(value);
    });

    html! {
        <div class={css!(r#"
            padding:24px;
            position:absolute;
            bottom: 0;
            left:0;
            right: 0;
        "#)}>
            <form onsubmit={handle_submit}>
            <input
                class={css!(r#"
                background-color: #40414f;
                width: 90%;
                padding: 12px;
                color:white;
                font-size: 1.5em;
                border-radius: 5px;
                border: none;
                border-color: none;
                margin:12px;
                outline: none;
                box-shadow: 0 0 8px 0 rgba(0, 0, 0.25);
                resize: none;
                "#)}
                rows='1'
                id="text"
                value={props.input.clone()}
                onchange={handle_change}
            />
            </form>
        </div>
      }
 }
  

pub fn toggle_disable(){
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    let _ = document.get_element_by_id( "text").unwrap().toggle_attribute("disabled");
}