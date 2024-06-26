use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SaveCheckDialogProps {
    pub on_close: Callback<()>,
}

#[function_component(SaveCheckDialog)]
pub fn save_check_dialog(props: &SaveCheckDialogProps) -> Html {
    let roll_result = use_state(|| None);

    let roll_d20 = {
        let roll_result = roll_result.clone();
        Callback::from(move |_| {
            let roll = rand::random::<u8>() % 20 + 1;
            roll_result.set(Some(roll));
        })
    };

    html! {
        <div>
            <h2>{ "Save Check" }</h2>
            <p>{ format!("Roll result: {:?}", *roll_result) }</p>
            <button onclick={roll_d20}>{ "Roll D20" }</button>
            <button onclick={props.on_close.clone()}>{ "Close" }</button>
        </div>
    }
}