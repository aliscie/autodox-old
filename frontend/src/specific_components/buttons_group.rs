use rand::seq::SliceRandom;
use yew::prelude::*;
use yew::{function_component, html, Html};

#[function_component(ButtonsGroup)]
pub fn buttons_group() -> Html {
    let categories: UseStateHandle<Vec<&str>> = use_state(|| vec!["work", "school"]);

    let _categories = categories.clone();
    let on_click = Callback::from(move |e: MouseEvent| {
        let mut new_categories = (*_categories).clone();
        let b = vec!["new"];
        new_categories.extend(b);
        _categories.set(new_categories.to_vec())
    });

    let on_files = Callback::from(move |e: MouseEvent| {});
    let vs = vec!["#937DC2", "#C689C6", "#FFABE1", "#fffc9b", "#FFE6F7"];

    html! {
        <div class="buttons_group_class">
            {
                (*categories).clone().into_iter().map(|name| {
                    html!{
                        <span
                            style={format!("background:{};", vs.choose(&mut rand::thread_rng()).unwrap())}
                            onclick={on_files} class="btn"
                        >
                            {&name}
                        </span>
                    }
                }).collect::<Html>()
            }
            <span onclick={on_click} class="btn"><i class="fa fa-plus"></i></span>
        </div>
    }
}
