use crate::bars::GameplayUI;
use crate::time::ControlledTime;
use crate::ScoreResource;
use bevy::prelude::*;

fn setup_ui(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let color = color_materials.add(Color::NONE);

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                // position: UiRect {
                left: Val::Px(10.),
                top: Val::Px(10.),
                // ..default()
                // },
                ..default()
            },
            // border_color: color,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Time: 0.0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ),
                    ..default()
                })
                .insert(TimeText);
        })
        .insert(GameplayUI);

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                left: Val::Px(10.),
                bottom: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn(TextBundle {
                    text: Text::from_section(
                        "Score: 0, Corrects: 0, Fails: 0",
                        TextStyle {
                            font_size: 40.0,
                            font: font.clone(),
                            color: Color::rgb(0.8, 0.8, 0.8),
                        },
                    ),
                    ..default()
                })
                .insert(ScoreText);
        })
        .insert(GameplayUI);
}

#[derive(Component)]
struct TimeText;

fn update_time_text(time: Res<ControlledTime>, mut query: Query<(&mut Text, &TimeText)>) {
    // Song starts 3 seconds after real time
    let secs = time.seconds_since_startup() - 3.;

    // Don't do anything before the song starts
    if secs < 0. {
        return;
    }

    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!("Time: {:.2}", secs);
    }
}

#[derive(Component)]
struct ScoreText;

fn update_score_text(score: Res<ScoreResource>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, _marker) in query.iter_mut() {
        text.sections[0].value = format!(
            "Score: {}, PG: {} GR: {}: GD: {}: BD: {}: PR: {}, Fails: {}",
            score.score(),
            score.pgreats,
            score.greats,
            score.goods,
            score.bads,
            score.poors,
            score.fails()
        );
    }
}

fn despawn_ui(mut commands: Commands, query: Query<(Entity, &GameplayUI)>) {
    for (entity, _) in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct UIPlugin<S: States> {
    pub state: S,
}

impl<S: States> Plugin for UIPlugin<S> {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, setup_ui.run_if(in_state(self.state.clone())));
        app.add_systems(OnEnter(self.state.clone()), setup_ui);
        app.add_systems(
            Update,
            (update_time_text, update_score_text).run_if(in_state(self.state.clone())),
        );
        app.add_systems(OnExit(self.state.clone()), despawn_ui);
    }
}
