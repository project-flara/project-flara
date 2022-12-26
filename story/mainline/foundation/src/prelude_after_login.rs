use bevy::{
    ecs::system::{BoxedSystem, IntoSystem},
    prelude::*,
};
use bevy_rpg::{characters::prelude::*, d, dialog::StyleDefaults, Dialogs};
use framework::Story;
pub struct FoundationIntroduction;

impl Story for FoundationIntroduction {
    fn id(&self) -> String {
        String::from("flara/mainline/foundation/prelude")
    }

    fn name(&self) -> String {
        String::from("Prelude")
    }

    fn author(&self) -> String {
        String::from("Fiana Fortressia")
    }

    fn license(&self) -> String {
        String::from("CC-BY-SA-4.0")
    }

    fn run(&self) -> BoxedSystem {
        fn run(commands: Commands, font: Res<AssetServer>) {
            let text_style = TextStyle {
                font: font.load("NotoSans-Regular.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            };
            let mut dialogs = Dialogs::new(StyleDefaults {
                text: text_style.clone(),
            });

            let player: PC = dialogs.single("You").into();

            let owner: PC = dialogs.single("").into();
            dialogs.add([
                d((&owner, "Hii!")),
                d((&owner, "So you're here now!")),
                d((&owner, "So that means")),
                d((&owner, "But si")),
            ]);

            dialogs.start(commands);
        }

        Box::new(IntoSystem::into_system(run))
    }
}
