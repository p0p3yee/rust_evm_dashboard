use yew::prelude::*;

use crate::{types::Endpoint};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub epl: Vec<Endpoint>,
    // pub callback: Callback<i32>
}

#[function_component(EndpointList)]
pub fn endpoint_table(props: &Props) -> Html {
    html! {
        <table class="table is-fullwidth is-hoverable has-background-light">
            <tbody>
                {
                    for props.epl.iter().map(|data| {
                        html! {
                            <tr>
                                <th class="is-size-4">
                                <span class="icon-text">
                                    <span class="icon">
                                        <i class="fa fa-plug"></i>
                                    </span>
                                    <span>
                                    {
                                        data.name.clone()
                                    }
                                    </span>
                                </span>
                                </th>
                            </tr>
                        }
                    })
                }
            </tbody>
        </table>
    }
}