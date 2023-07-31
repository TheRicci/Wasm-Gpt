use leptos::*;
use leptos_meta::*;

mod components;
use components::chat_area::ChatArea;
use components::chat_input::{ChatInput};

use crate::api::request;
use crate::model::chatlog::{ChatLog, Log};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (chat_log, set_log) = create_signal(cx, ChatLog::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Log {
            message: new_message.clone(),
            user: true,
        };
        set_log.update(move |c| {
            c.chat.push(user_message);
        });
        request(cx, chat_log.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            toggle_disable();
            let model_message = Log {
                message: String::from("..."),
                user: false,
            };

            set_log.update(move |c| {
                c.chat.push(model_message);
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_log.update(move |c| {
                c.chat.last_mut().unwrap().message = response;
            });
            toggle_disable();
        }
    });

    view! { cx,
        <html class="dark">
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        
        <Title text="Leptos-GPT"/>
        
        <ChatArea chat_log/>
        <ChatInput set_log send/>
        </html>
    }
}

pub fn toggle_disable(){
    let document = web_sys::window()
        .unwrap()
        .document()
        .unwrap();

    let _ = document.get_element_by_id("text").unwrap().toggle_attribute("disabled");
}