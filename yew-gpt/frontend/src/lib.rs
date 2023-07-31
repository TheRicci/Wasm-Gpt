use yew::prelude::*;
use stylist::yew::{styled_component,Global};
use std::ops::Deref;

mod components;
use components::chat_message::{ChatMessage,ChatLog};
use components::side_menu::SideMenu;
use components::chat_input::{ChatInput,toggle_disable};

use reqwasm::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
//use gloo::console::log;

struct ScopeCall<F: FnMut()> {
    c: F
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

macro_rules! defer {
    ($e:expr) => (
        let _scope_call = ScopeCall { c: || -> () { $e; } };
    )
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub message: String,
}
#[derive(Serialize, Deserialize)]
struct ResponseData {
    pub data: Response,
}

#[derive(Clone)]
struct Data {
    pub input: String,
    pub chat_log: Vec::<ChatLog>,
    pub previous_interaction: Option<Vec<serde_json::Value>>,
}

#[styled_component(App)]
pub fn app() -> Html {
    let state = use_state(|| Data{input:"".to_owned(),
        chat_log: vec![ChatLog{user: "gpt".to_owned(),
        message:html!(<p>{"How can i help you today?"}</p>)}],
        previous_interaction: None,

    });

    let cloned_state = state.clone();
    let clicked = Callback::from(move |()| {
        cloned_state.set(Data{input:"".to_owned(),
        chat_log: vec![ChatLog{user: "gpt".to_owned(),
        message:html!(<p>{"How can i help you today?"}</p>)}],
        previous_interaction: None
    }
    )});

    let cloned_state = state.clone();
    let input_changed = Callback::from(move |input| {
        cloned_state.set(Data {
            input,
            ..cloned_state.deref().clone()
        });
    });

    let cloned_state = state.clone();
    let on_submit = Callback::from(move |_| {
        defer!(toggle_disable());
        if cloned_state.input.split_whitespace().next() == None{return ()}

        cloned_state.set(Data {
            input: "thinking...".to_string(),
            ..cloned_state.deref().clone()
        });

        let cloned_state2 = cloned_state.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // parse new input to a serelialized json object
            let new_input = json!({"messanger":"user","message": cloned_state2.input.clone()});

            let body = match cloned_state2.previous_interaction.clone(){
                // if there is a previous conversation append those already json serialized objects with the new input
                Some(previous_interactions) => {
                    json!({
                        "inputs": [previous_interactions[0],previous_interactions[1],new_input]
                    })
                }
                None => {
                    json!({
                        "inputs": [new_input.clone()]
                    })
                }
            };

            //log!(body.to_string().clone());
            let resp = Request::post("http://127.0.0.1:8000/request")
            .header("content-type", "text/plain")
            .body(body.to_string())
            .send()
            .await
            .unwrap()
            .json::<ResponseData>()
            .await
            .unwrap();
            
            let msg = resp.data.message.clone().split("\n").into_iter().map(|line| html!(<p>{line}</p>)).collect::<Html>();

            // save the history of the conversation as HTML objects to show on the website
            let mut chat_vec = cloned_state2.chat_log.clone();
            chat_vec.push(ChatLog { user: "me".to_owned(), message: html!(<p style="word-wrap:break-word">{cloned_state2.input.clone()}</p>)});
            chat_vec.push(ChatLog { user: "gpt".to_owned(), message: html!(msg) });

            // sereliaze the AI response and append with the already sereliazed new_input json object to save them previous messages 
            // only saves the previous conversation to save tokens usage on the API
            let api_vec = Vec::from([new_input.clone(),json!({"messanger":"assistant","message": resp.data.message.clone()})]);
        
            cloned_state2.set(Data {
                input: "".to_owned(),
                chat_log: chat_vec,
                previous_interaction: Some(api_vec)
            });
        });
    });
   
    html! {
        <>
            <Global css={css!(r#"
                html {
                    box-sizing: border-box;
                    font-size: 16px;
                
                    -webkit-box-sizing: border-box;
                    -moz-box-sizing: border-box;
                            box-sizing: border-box;
                
                    font-family: helvetica;
                }
                
                *,
                *::before,
                *::after {
                    -webkit-box-sizing: inherit;
                        -moz-box-sizing: inherit;
                            box-sizing: inherit;
                }
                
                *, *:before, *:after {
                    box-sizing: inherit;
                }
                
                body, h1, h2, h3, h4, h5, h6, p, ol, ul {
                    margin: 0;
                    padding: 0;
                    font-weight: normal;
                }
                
                ol, ul {
                    list-style: none;
                }
                
                img {
                    max-width: 100%;
                    height: auto;
                }
            "#)} />
        <div>
            <div class={css!(r#"
                text-align: center;
                display: flex;
                background-color: #282c34;
                color:white;
                position: absolute;
                top:0;
                bottom: 0;
                right: 0;
                left: 0;
            "#)}>

                <SideMenu onclick={clicked}></SideMenu>
                <section class={css!(r#"
                    flex: 1;
                    position: relative;
                    overflow-y:auto;
                "#)}>
                    <div class={css!(r#"
                        text-align: left;
                        white-space: "pre-wrap";
                    "#)}>
                        {state.clone().chat_log.iter().map(|c| html!{<ChatMessage message={c.message.clone()} user={c.user.clone()} />} ).collect::<Html>()}
                    </div>
                    <div class={css!(r#"
                        padding:40px;
                    "#)}></div>
                    <div class={css!(r#"
                        padding:24px;
                        position:sticky;
                        top: 100%;
                    "#)}>
                        <ChatInput onsubmit={on_submit} onchange={input_changed} input={state.clone().input.clone()}></ChatInput>
                    </div>
                </section>
            </div>
        </div>
    </>
    }
}

