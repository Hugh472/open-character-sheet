use yew::prelude::*;
use crate::components::CastSpellDialog;

#[derive(Properties, PartialEq)]
pub struct SpellSheetProps {
    pub character_name: String,
}

#[function_component(SpellSheet)]
pub fn spell_sheet(props: &SpellSheetProps) -> Html {
    let show_cast_spell_dialog = use_state(|| None);

    let toggle_cast_spell_dialog = {
        let show_cast_spell_dialog = show_cast_spell_dialog.clone();
        Callback::from(move |spell: Option<String>| {
            show_cast_spell_dialog.set(spell);
        })
    };

    html! {
        <div>
            <h1>{ format!("{}'s Spell Sheet", props.character_name) }</h1>
            <div>
                <h2>{ "Known Spells" }</h2>
                <ul>
                    // Example spells
                    <li>
                        { "Fireball" }
                        <button onclick={toggle_cast_spell_dialog.reform(|_| Some("Fireball".into()))}>{ "Cast" }</button>
                    </li>
                    <li>
                        { "Magic Missile" }
                        <button onclick={toggle_cast_spell_dialog.reform(|_| Some("Magic Missile".into()))}>{ "Cast" }</button>
                    </li>
                </ul>
            </div>

            if let Some(spell_name) = &*show_cast_spell_dialog {
                <CastSpellDialog spell_name={spell_name.clone()} on_close={toggle_cast_spell_dialog.reform(|_| None)} />
            }
        </div>
    }
}