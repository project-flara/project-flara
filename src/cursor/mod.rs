use bevy::{
    prelude::*, render::camera::RenderTarget, transform::TransformSystem,
};

use crate::invisible_to_focus::InvisibleToFocus;
pub struct CustomCursor;

impl Plugin for CustomCursor {
    fn build(&self, app: &mut App) {
        app.add_startup_system(Self::setup).add_system(Self::update);
    }
}

impl CustomCursor {
    pub fn setup(
        mut windows: ResMut<Windows>,
        mut commands: Commands,
        server: Res<AssetServer>,
    ) {
          for window in windows.iter_mut() {
                   window.set_cursor_visibility(false);
               }
        
        commands.spawn((
            ImageBundle {
                style: Style {
                    position: UiRect::all(Val::Auto),
                    position_type: PositionType::Absolute,
                    size: Size::new(Val::Px(36.0), Val::Px(36.0)),
                    ..default()
                },

                image: UiImage(server.load("app/cursor.png")),
                ..default()
            },
            Cursor,
            InvisibleToFocus,
        ));
    }

    pub fn update(
        // need to get window dimensions
        wnds: Res<Windows>,
        // query to get camera transform
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
        mut cursor: Query<
            &mut Style,
            (With<Cursor>, Without<Camera>, Without<MainCamera>),
        >,
    ) {
        // get the camera info and transform
        // assuming there is exactly one main camera entity, so query::single() is OK
        let (camera, camera_transform) = q_camera.single();

        // get the window that the camera is displaying to (or the primary window)
        let wnd = if let RenderTarget::Window(id) = camera.target {
            wnds.get(id).unwrap()
        } else {
            wnds.get_primary().unwrap()
        };

        // check if the cursor is inside the window and get its position
        if let Some(screen_pos) = wnd.cursor_position() {
            let mut cursor = cursor.single_mut();
            
            cursor.position.left = Val::Px(screen_pos.x);
            cursor.position.top = Val::Px(wnd.height() - screen_pos.y);
        }
    }
}

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct MainCamera;
