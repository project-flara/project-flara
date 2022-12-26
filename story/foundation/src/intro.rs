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

            let owner: PC = dialogs.single("").into();
            let mom: PC = dialogs.single("'s Mom").into();

            dialogs.add([
                d((&owner, "Wait, Mom and Dad are leaving?")),
                d((&owner, "Mom!")),
                d((&mom, "Yes, honey?")),
                d((&owner, "Where are you two going?")),
                d((&mom, "We have an important meeting to attend. Take care of your little sister, okay?")),
                d((&owner, "Does it have to do with our family businesses?")),
                d((&mom, "Yes. Mom and Dad will meet with the companies' boards. We will be home by 12 AM.")),
                d((&owner, "Okay, Mom and Dad! Please take care of yourselves!")),
                d((&mom, "We'll take care of ourselves, honey.")),
                //////////////////////////////////////////////////////
                d((&owner, "I couldn't imagine that was the last time I was able to meet my parents.")),
                d((&owner, "Fate is evil sometimes.")),
                /////////////////////////////////////////////////////
                d((&owner, "My parents were good people. ")),
                d((&owner, "But ")),
                d((&owner, "From now on, the responsibilities for the businesses under the family will be placed on my shoulders.")),
                d((&owner, "I look forward for the cooperation and collaboration with all of you.")),
                /////////////////////////////////////////////////////
                d((&owner, "All the money I have now. It feels nothing to me.")),
                d((&owner, "It doesn't feel useful for me in any way")),
                d((&owner, "It's true. Money doesn't give you happiness.")),
                d((&owner, "It just gives you even more stress and greediness.")),
                d((&owner, "Maybe, I should help others..")),
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
