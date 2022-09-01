use yew::prelude::*;

use crate::{types::Endpoint, hooks::use_setting_context};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub epl: Vec<Endpoint>,
    pub on_delete_callback: Callback<i32>,
    pub on_select_callback: Callback<i32>,
}

#[function_component(EndpointList)]
pub fn endpoint_table(props: &Props) -> Html {
    let setting_ctx = use_setting_context();
    html! {
        <table class="table is-fullwidth is-hoverable has-background-light">
            <tbody>
                {
                    for props.epl.iter().map(|data| {
                        let now_id = (*data).id;
                        let delete_cb = props.on_delete_callback.clone();
                        let select_cb = props.on_select_callback.clone();
                        let on_delete = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            delete_cb.emit(now_id)
                        });
                        let on_select = Callback::from(move |e: MouseEvent| {
                            e.prevent_default();
                            select_cb.emit(now_id)
                        });
                        html! {
                            <tr class="is-size-4">
                                <th>
                                    <span class="icon-text">
                                        <span class="icon">
                                            <i class="fa fa-plug"></i>
                                        </span>
                                        <span> { data.name.clone() } </span>
                                    </span>
                                </th>
                                <th class="has-text-right">
                                    <button 
                                        class="button is-success is-outlined"
                                        onclick={on_select}
                                        disabled = {setting_ctx.id == now_id}
                                    >     
                                        {
                                            if setting_ctx.id == now_id {
                                                "Selected"
                                            } else {
                                                "Select"
                                            }
                                        }
                                    </button>
                                    <button class="button is-danger is-outlined ml-1" onclick={on_delete}>
                                        <span class="icon is-small">
                                            <i class="fas fa-times"></i>
                                        </span>
                                    </button>
                                </th>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}