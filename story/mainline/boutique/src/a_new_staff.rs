use bevy::{
    ecs::system::{BoxedSystem, IntoSystem},
    prelude::*,
};
use bevy_rpg::{characters::prelude::*, d, dialog::StyleDefaults, Dialogs};
use framework::Story;
pub struct ANewStaff;

impl Story for ANewStaff {
    fn id(&self) -> String {
        String::from("flara/mainline/foundation/prelude")
    }

    fn name(&self) -> String {
        String::from("A New Staff")
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

            let unknown: PC = dialogs.single("").into();
            let yuki: PC = dialogs.single("???").into();
            dialogs.add([d((&unknown, "Pleace accept me!"))]);

            dialogs.start(commands);
        }

        Box::new(IntoSystem::into_system(run))
    }
}
