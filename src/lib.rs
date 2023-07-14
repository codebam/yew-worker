use worker::*;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}

#[event(fetch)]
async fn main(req: Request, env: Env, ctx: worker::Context) -> Result<Response> {
    Response::from_html(yew::ServerRenderer::<App>::new().render().await)
}
