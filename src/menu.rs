use crate::consts::*;
use crate::types::load_config;
use bevy::prelude::*;
use std::fs::read_dir;

#[derive(Resource)]
struct ButtonMaterials {
    none: BackgroundColor,
    normal: BackgroundColor,
    hovered: BackgroundColor,
    pressed: BackgroundColor,
    font: Handle<Font>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
        ButtonMaterials {
            none: Color::NONE.into(),
            normal: Color::rgb(0.15, 0.15, 0.15).into(),
            hovered: Color::rgb(0.25, 0.25, 0.25).into(),
            pressed: Color::rgb(0.35, 0.35, 0.35).into(),
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

#[derive(Component)]
enum MenuButton {
    PlaySong(String),
}
impl MenuButton {
    fn name(&self) -> String {
        match self {
            Self::PlaySong(song) => format!("Play song: {}", song),
        }
    }
}

#[derive(Component)]
struct MenuUI;

fn setup_menu(mut commands: Commands, button_materials: Res<ButtonMaterials>) {
    // Make list of buttons
    let mut buttons: Vec<MenuButton> = get_songs()
        .iter()
        .map(|name| MenuButton::PlaySong(name.clone()))
        .collect();

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexStart,
                ..default()
            },
            background_color: button_materials.none.into(),
            ..default()
        })
        .insert(MenuUI)
        .with_children(|parent| {
            for button in buttons {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: button_materials.normal.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text::from_section(
                                button.name(),
                                TextStyle {
                                    font: button_materials.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                    ..default()
                                },
                            ),
                            ..default()
                        });
                    })
                    .insert(button);
            }
        });
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &MenuUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn button_color_system(
    button_materials: Res<ButtonMaterials>,
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut color) in query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *color = button_materials.pressed;
            }
            Interaction::Hovered => {
                *color = button_materials.hovered;
            }
            Interaction::None => {
                *color = button_materials.normal;
            }
        }
    }
}

fn button_press_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    // state: ResMut<State<MyAppState>>,
    mut next_state: ResMut<NextState<MyAppState>>,
) {
    for (interaction, button) in query.iter() {
        if *interaction == Interaction::Pressed {
            match button {
                MenuButton::PlaySong(song) => {
                    let config = load_config(song, &asset_server);
                    commands.insert_resource(config);
                    next_state.set(MyAppState::InGame);
                }
            }
        }
    }
}

// probably eventually want to store song data in a DB to avoid having to parse the whole directory all the time
pub fn get_songs() -> Vec<String> {
    let song_dirs = read_dir("assets/songs").expect("Failed to read songs dir");

    let mut vec: Vec<String> = Vec::new();
    for song_dir in song_dirs {
        let paths = read_dir(song_dir.unwrap().path()).expect("Failed to read song dir");

        for path in paths {
            let path = path.expect("Failed to unwrap path").path();

            if "bms" == path.as_path().extension().unwrap() {
                let path_stripped_prefix = path
                    .as_path()
                    .strip_prefix("assets/")
                    .unwrap()
                    .to_str()
                    .expect("Failed to get path")
                    .to_string();

                vec.push(path_stripped_prefix);
            }
        }
    }

    vec
}

pub struct MenuPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for MenuPlugin<S> {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonMaterials>();
        // app.add_systems(Startup, setup_menu.run_if(in_state(self.state.clone())));
        app.add_systems(OnEnter(self.state.clone()), setup_menu);
        app.add_systems(OnExit(self.state.clone()), despawn_menu);
        app.add_systems(
            Update,
            (button_color_system, button_press_system).run_if(in_state(self.state.clone())),
        );
    }
}
