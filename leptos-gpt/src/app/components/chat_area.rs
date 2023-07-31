use leptos::{*, html::Div};
use crate::model::chatlog::ChatLog;

const USER_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 mr-[10%] mt-10 rounded-lg self-end bg-blue-500 text-white";
const MODEL_MESSAGE_CLASS: &str = "max-w-md p-4 mb-5 ml-[10%] mt-10 rounded-lg self-start bg-gray-200 text-black";

#[component]
pub fn ChatArea(cx: Scope, chat_log: ReadSignal<ChatLog>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>(cx);
    create_effect(cx, move |_| {
      chat_log.get();
      if let Some(div) = chat_div_ref.get() {
        div.set_scroll_top(div.scroll_height());
      }
    });
    view! { cx,
        <div class="h-screen p-4 pl-10 w-full flex flex-col overflow-y-auto bg-[#282c34]" node_ref=chat_div_ref>
          {move || chat_log.get().chat.iter().map(move |message| {
              let class_str = if message.user { USER_MESSAGE_CLASS } else { MODEL_MESSAGE_CLASS };
              view! {cx,
                <div class={class_str}>
                  {message.message.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}
