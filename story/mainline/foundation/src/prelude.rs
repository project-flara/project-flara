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

            dialogs.add([
                d((&player, "Thank you everyone!")),
                ////////////////////////////////////////////////////
            ]);
            let yuki: PC = dialogs.single("Yuki").into();
            let ayane: PC = dialogs.single("Ayane").into();
            dialogs.add([
                d((&yuki, "You can't change the past, but remember that the future will always be there for you to change it.")),
                d((&ayane, "Life is like a novel. All the challanges make up the great colors!"))
            ]);

            dialogs.start(commands);
        }

        Box::new(IntoSystem::into_system(run))
    }
}
