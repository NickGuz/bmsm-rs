use crate::consts::MyAppState;
use crate::score::ScoreResource;
use bevy::prelude::*;

macro_rules! spawn_text_entity {
    ($parent:expr, $asset_server:expr, $text:expr, $score:expr, $font_size:expr, $margin_px:expr) => {
        $parent.spawn((
            TextBundle::from_section(
                format!("{}: {}", $text, $score),
                TextStyle {
                    font: $asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: $font_size,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect {
                    left: Val::Percent(5.),
                    right: Val::Percent(5.),
                    top: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                },
                ..default()
            }),
            Label,
        ));
    };
}

#[derive(Component)]
struct ResultsUI;

fn setup_results_screen(
    mut commands: Commands,
    score: Res<ScoreResource>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                // justify_content: JustifyContent::SpaceBetween,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        })
        .insert(ResultsUI)
        .with_children(|parent| {
            spawn_text_entity!(parent, asset_server, "PGREAT", score.pgreats, 30.0, 1.0);
            spawn_text_entity!(parent, asset_server, "GREAT", score.greats, 30.0, 1.0);
            spawn_text_entity!(parent, asset_server, "GOOD", score.goods, 30.0, 1.0);
            spawn_text_entity!(parent, asset_server, "BAD", score.bads, 30.0, 1.0);
            spawn_text_entity!(parent, asset_server, "POOR", score.poors, 30.0, 1.0);
        });
}

fn despawn_menu(mut commands: Commands, query: Query<(Entity, &ResultsUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn go_to_song_select(
    mut next_state: ResMut<NextState<MyAppState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) || mouse_input.just_pressed(MouseButton::Left) {
        next_state.set(MyAppState::SongSelect); // TODO This should really be SongSelect
    }
}

fn reset_score(mut score: ResMut<ScoreResource>) {
    score.reset();
}

pub struct ResultsPlugin<S: States> {
    pub state: S,
}
impl<S: States> Plugin for ResultsPlugin<S> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(self.state.clone()), setup_results_screen);
        app.add_systems(
            Update,
            go_to_song_select.run_if(in_state(self.state.clone())),
        );
        app.add_systems(OnExit(self.state.clone()), (despawn_menu, reset_score));
    }
}
