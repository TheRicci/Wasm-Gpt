use leptos::{*, html::Input};
use crate::model::chatlog::ChatLog;

#[component]
pub fn ChatInput(cx: Scope, set_log:WriteSignal<ChatLog>, send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>(cx);
    view!{ cx,
        <div class="h-24 w-full fixed bottom-0 flex justify-center items-center p-5 bg-[#282c34]">
            <button class=" flex p-6 bg-[#282c34] text-slate-300 rounded-lg shadow-[#8b8b8b] shadow-inner" on:click=move |ev|
                {
                    ev.prevent_default();
                    set_log.set(ChatLog::new());
                }>
                    {"Clear Chat"}
            </button>
            <form class="w-2/3 flex justify-center items-center gap-4" on:submit=move |ev| 
                    {
                        ev.prevent_default();
                        let input = input_ref.get().unwrap();
                        send.dispatch(input.value());
                        input.set_value("");
                    }>
                        <input class="w-2/3 p-4 rounded-lg bg-[#40414f] text-white shadow-[#000000] shadow-lg" id="text" type="text" node_ref=input_ref/>
            </form>
        </div>
    }
}

