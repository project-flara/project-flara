use bevy::{
    ecs::system::{BoxedSystem, IntoSystem},
    prelude::*,
};
use bevy_rpg::{characters::prelude::*, d, dialog::StyleDefaults, Dialogs};
use framework::Story;
pub struct UnseenCircumstances;

impl Story for UnseenCircumstances {
    fn id(&self) -> String {
        String::from("flara/mainline/foundation/prelude")
    }

    fn name(&self) -> String {
        String::from("UnseenCircumstances")
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

            let unknown: PC = dialogs.single("???").into();
            let yuki: PC = dialogs.single("???").into();
            dialogs.add([
                d((&unknown, "When primitive humans started, the world was cruel to them.")),
                d((&unknown, "The beasts, the nights, the scarce, the unseen circumstances.")),
                d((&unknown, "So they all stand up to fight them.")),
                d((&unknown, "Anything can be defeated if we are together.")),
                d((&unknown, "But here we are, together.")),
                d((&unknown, "Yet, together, is imaginary.")),
                d((&unknown, "I was always left alone.")),
                d((&unknown, "Left to face the unfair jungle, without help.")),
                d((&unknown, "If I could make a single wish come to true.")),
                d((&unknown, "I just want...")),
                d((&unknown, "Someone I could call a friend")),
                ////////////////////////////////////////////////////
                // S
                ////////////////////////////////////////////////////
                d((&yuki, "Oh, is there a mailperson?")),
                d((&yuki, "Oh my gosh. It's already 9AM. I think I slept for too long.")),
                d((&yuki, "Well, at least I no longer have school today!")),
                d((&yuki, "Which means, I can work again on my OC.")),
                d((&yuki, "But first, let's check out the mail they dropped just now.")),
                d((&yuki, "I'm worried they'll have to wait for too long.")),
              
            ]);
            let mailperson: PC = dialogs.single("Mailperson").into();
            dialogs.add([  d((&yuki, "Hiii!")),
                d((&mailperson, "Are you Ms. [surname]?")),
                d((&yuki, "Yes, I am! Thank you very much!")),
                d((&mailperson, "You're welcome. From who is it?")),
            d((&mailperson, "Um, it's from Middle School. ")),]);

            dialogs.add([ d((&yuki, "Hm, let's see what it is inside.")),
            d((&yuki, "I left my middle school just before graduation, just after the finals. So it must be my yearbook.")),
            d((&yuki, "Oh my gosh, it's fortunate that when I left, I already did every exams and the finals, so they still would give me the certificates.")),
            d((&yuki, "Aww okay!")),
            d((&yuki, "Oh my God! This thing is just sooo cute!")),
            d((&yuki, "Whoever made this yearbook is so cool!")),
            d((&yuki, "Ther"))]);

            let unknown: PC = dialogs.single("unknown").into();
            let ayane: PC = dialogs.single("Ayane").into();
            dialogs.add([
                d((&unknown, "You can't change the past, but remember that the future will always be there for you to change it.")),
                d((&ayane, "Life is like a novel. All the challanges make up the great colors!"))
            ]);

            dialogs.start(commands);
        }

        Box::new(IntoSystem::into_system(run))
    }
}
