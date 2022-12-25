use bevy::prelude::*;
use dlopen::wrapper::Container;
use dlopen::wrapper::WrapperApi;
use dlopen_derive::WrapperApi;
use framework::Chapter;
#[derive(WrapperApi)]
pub struct MainStoryPluginAPI {
    chapter: fn() -> Box<dyn Chapter>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Component)]
pub enum MainStory {
    // Flara Foundation
    Foundation,
}

impl MainStory {
    pub const VARIATIONS: [MainStory; 1] = [Self::Foundation];
    pub fn module_name(&self) -> String {
        "libflarastory_".to_string() + &self.to_string().to_lowercase() + "." + dynamic_module()
    }
}

impl ToString for MainStory {
    fn to_string(&self) -> String {
        (match self {
            Self::Foundation => "foundation",
        })
        .to_string()
    }
}

///
/// # Safety
/// As unsafety as [dlopen::Container::load()]
///
///  
pub unsafe fn load(
    story: &MainStory,
) -> Result<Container<MainStoryPluginAPI>, dlopen::Error> {
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

        assert_eq!(first.name(), String::from("Foundation Prelude"));
        assert_eq!(first.author(), String::from("Fiana Fortressia"));
        assert_eq!(first.license(), String::from("CC-BY-SA 4.0"));
    }
}
