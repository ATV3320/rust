use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| Model{
        value: 65
    });
    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };
    html! {
        <div>
        <button{onclick}>{"click me"}</button>
        <p>{state.value}</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
