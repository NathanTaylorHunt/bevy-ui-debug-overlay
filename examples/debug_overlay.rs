//! Example demonstrating how to use the debug overlay

use bevy::{
    color::palettes::{basic::LIME, css::DARK_GRAY},
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    ui::widget::NodeImageMode,
};
use bevy_ui_debug_overlay::{UiDebugOverlay, UiDebugOverlayPlugin};
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            UiDebugOverlayPlugin::start_enabled().with_line_width(2.),
        ))
        .add_systems(Startup, (setup, debug_overlay_setup))
        .add_systems(Update, (update_scroll_position, toggle_debug_overlay))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2d, IsDefaultUiCamera, UiBoxShadowSamples(6)));

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(200.),
                        border: UiRect::all(Val::Px(2.)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.65, 0.65, 0.65)),
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Percent(100.),
                                flex_direction: FlexDirection::Column,
                                padding: UiRect::all(Val::Px(5.)),
                                row_gap: Val::Px(5.),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                        ))
                        .with_children(|parent| {
                            // text
                            parent.spawn((
                                Text::new("Text Example"),
                                TextFont {
                                    font_size: 25.0,
                                    ..default()
                                },
                            ));

                            // Debug overlay text
                            parent.spawn((Text::new("Press Space to toggle the debug overlay."),));

                            parent.spawn((Text::new(
                                "Press V to toggle the UI hierarchy's visibility.",
                            ),));
                            parent.spawn((Text::new(
                                "Press S to toggle outlines for hidden UI nodes.",
                            ),));
                            parent.spawn((Text::new(
                                "Press C to toggle outlines for clipped UI nodes.",
                            ),));
                        });
                });
            // right vertical fill
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Px(200.),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Scrolling list"),
                        TextFont {
                            font_size: 21.,
                            ..default()
                        },
                    ));
                    parent
                        .spawn((
                            Node {
                                flex_direction: FlexDirection::Column,
                                align_self: AlignSelf::Stretch,
                                height: Val::Percent(50.),
                                overflow: Overflow::scroll_y(),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.10, 0.10, 0.10)),
                        ))
                        .with_children(|parent| {
                            for i in 0..25 {
                                parent.spawn((Text(format!("Item {i}")),)).insert(
                                    PickingBehavior {
                                        should_block_lower: false,
                                        ..default()
                                    },
                                );
                            }
                        });
                });

            parent
                .spawn(Node {
                    left: Val::Px(210.),
                    bottom: Val::Px(10.),
                    position_type: PositionType::Absolute,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Px(200.0),
                                border: UiRect::all(Val::Px(20.)),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BorderColor(LIME.into()),
                            BackgroundColor(Color::srgb(0.8, 0.8, 1.)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                ImageNode::new(asset_server.load("branding/bevy_logo_light.png")),
                                Transform::from_rotation(Quat::from_rotation_z(0.25 * PI)),
                                BorderRadius::all(Val::Px(10.)),
                                Outline {
                                    width: Val::Px(2.),
                                    offset: Val::Px(4.),
                                    color: DARK_GRAY.into(),
                                },
                            ));
                        });
                });

            let shadow_style = BoxShadow {
                color: Color::BLACK.with_alpha(0.5),
                blur_radius: Val::Px(2.),
                x_offset: Val::Px(10.),
                y_offset: Val::Px(10.),
                ..default()
            };

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .insert(PickingBehavior::IGNORE)
                .with_children(|parent| {
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(100.0),
                                height: Val::Px(100.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(1.0, 0.0, 0.)),
                            shadow_style,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(20.),
                                    bottom: Val::Px(20.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.3, 0.3)),
                                shadow_style,
                            ));
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(40.),
                                    bottom: Val::Px(40.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(1.0, 0.5, 0.5)),
                                shadow_style,
                            ));
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(60.),
                                    bottom: Val::Px(60.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.0, 0.7, 0.7)),
                                BoxShadow::from(shadow_style),
                            ));
                            parent.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Percent(100.0),
                                    position_type: PositionType::Absolute,
                                    left: Val::Px(80.),
                                    bottom: Val::Px(80.),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(1.0, 0.9, 0.9, 0.4)),
                                BoxShadow {
                                    color: Color::BLACK.with_alpha(0.3),
                                    ..shadow_style
                                },
                            ));
                        });
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexStart,
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ImageNode::new(asset_server.load("branding/bevy_logo_dark_big.png"))
                                .with_mode(NodeImageMode::Stretch),
                            Node {
                                width: Val::Px(500.0),
                                height: Val::Px(125.0),
                                margin: UiRect::top(Val::VMin(5.)),
                                ..default()
                            },
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Node {
                                    display: Display::None,
                                    ..default()
                                },
                                Text::new("Bevy logo"),
                            ));
                        });
                });

            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    column_gap: Val::Px(10.),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                })
                .insert(PickingBehavior::IGNORE)
                .with_children(|parent| {
                    for (flip_x, flip_y) in
                        [(false, false), (false, true), (true, true), (true, false)]
                    {
                        parent.spawn((
                            ImageNode {
                                image: asset_server.load("branding/icon.png"),
                                flip_x,
                                flip_y,
                                ..default()
                            },
                            Node {
                                width: Val::Px(75.),
                                ..default()
                            },
                        ));
                    }
                });
        });
}

pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (mouse_wheel_event.x * 20., mouse_wheel_event.y * 20.),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

fn toggle_debug_overlay(
    input: Res<ButtonInput<KeyCode>>,
    mut debug_overlay: ResMut<UiDebugOverlay>,
    mut root_node_query: Query<&mut Visibility, (With<Node>, Without<Parent>)>,
) {
    if input.just_pressed(KeyCode::Space) {
        // The toggle method will enable the debug overlay if disabled and disable if enabled
        debug_overlay.toggle();
    }

    if input.just_pressed(KeyCode::KeyS) {
        // Toggle debug outlines for nodes with `ViewVisibility` set to false.
        debug_overlay.show_hidden = !debug_overlay.show_hidden;
    }

    if input.just_pressed(KeyCode::KeyC) {
        // Toggle outlines for clipped UI nodes.
        debug_overlay.show_clipped = !debug_overlay.show_clipped;
    }

    if input.just_pressed(KeyCode::KeyV) {
        for mut visibility in root_node_query.iter_mut() {
            // Toggle the UI root node's visibility
            visibility.toggle_inherited_hidden();
        }
    }
}

fn debug_overlay_setup(mut debug_overlay: ResMut<UiDebugOverlay>) {
    debug_overlay.enabled = true;
    debug_overlay.line_width = 2.;
}
