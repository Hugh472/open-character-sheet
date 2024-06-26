use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CastSpellDialogProps {
    pub spell_name: String,
    pub on_close: Callback<()>,
}

#[function_component(CastSpellDialog)]
pub fn cast_spell_dialog(props: &CastSpellDialogProps) -> Html {
    let roll_result = use_state(|| None);
    let save_dc = 15; // Example save DC

    let roll_d20 = {
        let roll_result = roll_result.clone();
        Callback::from(move |_| {
            let roll = rand::random::<u8>() % 20 + 1;
            roll_result.set(Some(roll));
        })
    };

    html! {
        <div>
            <h2>{ format!("Casting {}", props.spell_name) }</h2>
            <p>{ format!("Save DC: {}", save_dc) }</p>
            <p>{ format!("Roll result: {:?}", *roll_result) }</p>
            <button onclick={roll_d20}>{ "Roll D20" }</button>
            <button onclick={props.on_close.clone()}>{ "Close" }</button>
        </div>
    }
}