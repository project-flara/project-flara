use framework::{Chapter, Story};

pub mod a_new_staff;
pub mod unseen_circumstances;
#[no_mangle]
pub fn chapter() -> Box<dyn Chapter> {
    Box::new(FoundationChapter)
}

pub struct FoundationChapter;

impl Chapter for FoundationChapter {
    fn name(&self) -> String {
        String::from("Sakura Blossoms Everyday!")
    }

    fn author(&self) -> String {
        String::from("Fiana Fortressia")
    }

    fn license(&self) -> String {
        String::from("CC-BY-SA-4.0")
    }

    fn stories(&self) -> Vec<Box<dyn Story>> {
        vec![
            Box::new(unseen_circumstances::UnseenCircumstances),
            Box::new(a_new_staff::ANewStaff),
        ]
    }
}
