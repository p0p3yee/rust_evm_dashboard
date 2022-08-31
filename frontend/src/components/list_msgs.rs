use yew::prelude::*;

#[derive(Properties, Clone, PartialEq, Default)]
pub struct ListMsgsProps {
    pub is_error: bool,
    pub msg: String,
}

#[function_component(ListMessages)]
pub fn list_msgs(props: &ListMsgsProps) -> Html {
    let is_closed = use_state(|| false);
    let msg_class = if props.is_error {
        "notification is-danger"
    } else {
        "notification is-success"
    };

    let on_close_click = {
        let is_closed = is_closed.clone();
        Callback::from(move |_| {
            is_closed.set(true);
        })
    };

    if !props.msg.is_empty() && *is_closed == false{
        html! {
            <div class={ msg_class }>
                <button class="delete" onclick={ on_close_click }></button>
                <strong> { &props.msg } </strong>
            </div>
        }
    } else {
        html! {}
    }
}
