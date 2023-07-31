use yew::prelude::*;
use stylist::yew::styled_component;

#[derive(Properties, PartialEq, Clone)]
pub struct Props{
    pub onclick: Callback<()>,
}

#[styled_component(SideMenu)]
pub fn side_menu(props: &Props) -> Html {
  let onclick = props.onclick.clone();
  let handle_click = Callback::from(move |_| {
      onclick.emit(())
  });

    html! {
    <aside class={css!(r#"
      width: 260px;
      padding:10px;
      background-color: #202123;
    "#)}>
      <div class={css!(r#"      
        padding: 15px;
        border:1px solid white; 
        border-radius: 5px;
        text-align: center;
        transition: ease 0.25 all;
      
        &:hover{
          background-color: rgba(255, 100, 222, 0.1);
        }
      
        span{
          padding-left: 6px;
          padding-right: 12px;
        }
      "#)}  onclick={handle_click}>
        {"Clear Chat"}
      </div>
    </aside>
    }
}