use framework::Story;

pub mod intro;

#[no_mangle]
pub fn story() -> Vec<Box<dyn Story>> {
    vec![Box::new(intro::FoundationIntroduction)]
}
