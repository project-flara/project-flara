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
                d((&player, "I can't imagine it's already time for us to graduate!")),
                d((&player, "Huh, I still haven't got a job.")),
                d((&player, "Would you like to be interested in joining? ")),
                d((&player, "Eh?? What is it?")),
                d((&player, "It's basically a charity foundation. It's new, and it's made by the .")),
                d((&player, "Okay, let me check their website.")),
                d((&player, "R&D Consultant")),
                d((&player, "Okay, but let's sign up first."))
                ////////////////////////////////////////////////////
            ]);

            dialogs.start(commands);
        }

        Box::new(IntoSystem::into_system(run))
    }
}
