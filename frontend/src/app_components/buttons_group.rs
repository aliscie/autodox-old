use yew::prelude::*;
use yew::{function_component, html, Html};
use rand::seq::SliceRandom;


#[derive(Properties, PartialEq)]
pub struct Props {
    // pub id: u64,
}

#[function_component(ButtonsGroup)]
pub fn buttons_group(props: &Props) -> Html {
    let categories: UseStateHandle<Vec<&str>> = use_state(|| vec!["work", "school"]);

    let _categories = categories.clone();
    let onclick = Callback::from(move |e: MouseEvent| {
        let mut new_categories = (*_categories).clone();
        let b = vec!["new"];
        new_categories.extend(b);
        _categories.set(new_categories.to_vec())
    });
    let handle_files = Callback::from(move |e: MouseEvent| {});
    let vs = vec!["#937DC2","#C689C6","#FFABE1","#fffc9b", "#FFE6F7"];

    html! {
        <div class="buttons_group_class">
         {
            (*categories).clone().into_iter().map(|name| {
                html!{<span
                    style={format!("background:{};",
                    vs.choose(&mut rand::thread_rng()).unwrap()
                    )}
                    onclick={handle_files.clone()} class="btn">{&name}</span>}
            }).collect::<Html>()
        }
            <span {onclick} class="btn"><i class="fa fa-plus"></i></span>
        </div>
    }
}
