
use dlopen::wrapper::Container;
use dlopen::wrapper::WrapperApi;
use dlopen_derive::WrapperApi;
use framework::Chapter;

use framework::states::EventStory;
use framework::states::MainStory;
#[derive(WrapperApi)]
pub struct MainStoryPluginAPI {
    chapter: fn() -> Box<dyn Chapter>,
}

impl StoryDylib for MainStory {}
impl StoryDylib for EventStory {}
impl<A: StoryDylib> StoryDylib for &A {}
pub trait StoryDylib
where
    Self: std::fmt::Debug,
{
    fn module_name(&self) -> String
    where
        Self: ToString,
    {
        "libflarastory_".to_string()
            + &self.to_string().to_lowercase()
            + "."
            + dynamic_module()
    }
}

///
/// # Safety
/// As unsafety as [dlopen::wrapper::Container::load()]
///
///  
pub unsafe fn load<A>(
    story: A,
) -> Result<Container<MainStoryPluginAPI>, dlopen::Error>
where
    A: StoryDylib + std::fmt::Display,
{
    Container::load(story.module_name())
}
fn dynamic_module() -> &'static str {
    #[cfg(target_os = "macos")]
    return "dynlib";
    #[cfg(target_family = "unix")]
    return "so";
    #[cfg(target_os = "windows")]
    return "dll";
}
#[cfg(test)]
mod tests {
    use dlopen::wrapper::Container;

    use super::MainStoryPluginAPI;

    #[test]
    pub fn load_main_stories() {
        let cont: Container<MainStoryPluginAPI> =
            unsafe { Container::load("libflarastory_foundation.so") }
                .expect("Could not open library or load symbols");
        let stories = cont.chapter().stories();
        let first = stories.first().unwrap();

        assert_eq!(first.name(), String::from("Prelude"));
        assert_eq!(first.author(), String::from("Fiana Fortressia"));
        assert_eq!(first.license(), String::from("CC-BY-SA-4.0"));
    }
}

impl StoryDylib for framework::states::Story {
    fn module_name(&self) -> String
where {
        match self {
            Self::MainStory(story) => story.to_string(),
            Self::EventStory(story) => story.to_string(),
        }
    }
}
