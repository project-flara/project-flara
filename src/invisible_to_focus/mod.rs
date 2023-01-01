use bevy::{prelude::*, ui::ui_focus_system, utils::HashMap};

pub struct InvisibleToFocusPlugin;

impl Plugin for InvisibleToFocusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Map>();
        app.add_system_to_stage(
            CoreStage::PreUpdate,
            Self::turn_off_visibility
                .before(ui_focus_system)
                .before(Self::turn_on_visibility),
        )
        .add_system_to_stage(
            CoreStage::PreUpdate,
            Self::turn_on_visibility
                .after(ui_focus_system)
                .after(Self::turn_off_visibility),
        );
    }
}
#[derive(DerefMut, Deref, Resource, Default)]
pub struct Map(HashMap<Entity, ComputedVisibility>);
impl InvisibleToFocusPlugin {
    pub fn turn_off_visibility(
        mut visibilities: Query<
            (&mut ComputedVisibility, Entity),
            With<InvisibleToFocus>,
        >,
     
        mut map: ResMut<Map>,
    ) {
    
        for (mut visibility, entity) in visibilities.iter_mut() {
            map.insert(entity, visibility.clone());
            *visibility = ComputedVisibility::INVISIBLE;
        }
    }
    pub fn turn_on_visibility(
        mut visibilities: Query<
            (Entity, &mut ComputedVisibility),
            With<InvisibleToFocus>,
        >,
        mut res: ResMut<Map>,
    ) {
        for (entity, mut visibility) in visibilities.iter_mut() {
            *visibility = res.get(&entity).unwrap().clone();
        }
        res.clear();
    }
}

#[derive(Component)]
pub struct InvisibleToFocus;
