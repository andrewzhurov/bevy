//! Example demonstrating bordered UI nodes

use bevy::{color::palettes::css::*, ecs::spawn::SpawnIter, prelude::*};

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
};
use bevy_ecs::relationship::{AncestorIter, RelationshipSourceCollection};

fn debug_system(named_q: Query<(&Name, &Transform, &ComputedNode)>) {
    println!("in debug");
    for (name, tf, comp_node) in &named_q {
        let tn = tf.translation;
        let size = comp_node.size;
        let content_size = comp_node.content_size;
        println!("{name:?}: translation {tn:?}");
        println!("{name:?}: size {size:?}");
        println!("{name:?}: content_size {content_size:?}");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (debug_system, update_scroll_position))
        .run();
}

const LINE_HEIGHT: f32 = 21.;

/// Updates the scroll position of scrollable nodes in response to mouse input
pub fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
    names_q: Query<&Name>,
    child_of_query: Query<&ChildOf>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (mut dx, mut dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        if keyboard_input.pressed(KeyCode::ShiftLeft) || keyboard_input.pressed(KeyCode::ShiftRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                let ancestors_iter = (*entity)
                    .iter()
                    .chain(AncestorIter::new(&child_of_query, *entity));
                for ancestor_entity in ancestors_iter {
                    // This may mut scroll position multiple times,
                    // E.g., multiple pointers hovering over the same entity.
                    // TODO collect _distinct_ scroll targets, then mut
                    if let Ok(mut scroll_position) = scrolled_node_query.get_mut(ancestor_entity) {
                        // let name = names_q.get(ancestor_entity).unwrap();
                        // println!("found scroll target: {name:?}");
                        scroll_position.offset_x -= dx;
                        scroll_position.offset_y -= dy;
                        break;
                    }
                }
            }
        }
    }
}

fn setup(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2d);

    // labels for the different border edges
    let border_labels = [
        "None",
        "All",
        "Left",
        "Right",
        "Top",
        "Bottom",
        "Horizontal",
        "Vertical",
        "Top Left",
        "Bottom Left",
        "Top Right",
        "Bottom Right",
        "Top Bottom Right",
        "Top Bottom Left",
        "Top Left Right",
        "Bottom Left Right",
    ];

    // all the different combinations of border edges
    // these correspond to the labels above
    let borders = [
        UiRect::default(),
        UiRect::all(Val::Px(10.)),
        UiRect::left(Val::Px(10.)),
        UiRect::right(Val::Px(10.)),
        UiRect::top(Val::Px(10.)),
        UiRect::bottom(Val::Px(10.)),
        UiRect::horizontal(Val::Px(10.)),
        UiRect::vertical(Val::Px(10.)),
        UiRect {
            left: Val::Px(20.),
            top: Val::Px(10.),
            ..default()
        },
        UiRect {
            left: Val::Px(10.),
            bottom: Val::Px(20.),
            ..default()
        },
        UiRect {
            right: Val::Px(20.),
            top: Val::Px(10.),
            ..default()
        },
        UiRect {
            right: Val::Px(10.),
            bottom: Val::Px(10.),
            ..default()
        },
        UiRect {
            right: Val::Px(10.),
            top: Val::Px(20.),
            bottom: Val::Px(10.),
            ..default()
        },
        UiRect {
            left: Val::Px(10.),
            top: Val::Px(10.),
            bottom: Val::Px(10.),
            ..default()
        },
        UiRect {
            left: Val::Px(20.),
            right: Val::Px(10.),
            top: Val::Px(10.),
            ..default()
        },
        UiRect {
            left: Val::Px(10.),
            right: Val::Px(10.),
            bottom: Val::Px(20.),
            ..default()
        },
    ];

    // scrollbar friendly?
    // TODO add scroll Y to root component, as there's plenty of stuff now.
    // DONE fix image size (to get shrinked by padding + border)

    let borders_examples = (
        Name::new("Borders Example"),
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border,
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                            children![(
                                Node {
                                    width: Val::Px(10.),
                                    height: Val::Px(10.),
                                    ..default()
                                },
                                BackgroundColor(YELLOW.into()),
                            )]
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let non_zero = |x, y| x != Val::Px(0.) && y != Val::Px(0.);
    let border_size = move |x, y| {
        if non_zero(x, y) {
            f32::MAX
        } else {
            0.
        }
    };

    let borders_examples_rounded = (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            move |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                border,
                                margin: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            BorderRadius::px(
                                border_size(border.left, border.top),
                                border_size(border.right, border.top),
                                border_size(border.right, border.bottom,),
                                border_size(border.left, border.bottom),
                            ),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                            children![(
                                Node {
                                    width: Val::Px(10.),
                                    height: Val::Px(10.),
                                    ..default()
                                },
                                BorderRadius::MAX,
                                BackgroundColor(YELLOW.into()),
                            )],
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let empty_borders_examples = (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Node {
                                // width: Val::Px(50.),
                                // height: Val::Px(50.),
                                border,
                                margin: UiRect::all(Val::Px(20.)),
                                padding: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let text_borders_examples = (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Text::new("text\nlines"),
                            TextBackgroundColor::from(PURPLE),
                            TextShadow::default(),
                            Node {
                                border,
                                margin: UiRect::all(Val::Px(20.)),
                                padding: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let text_borders_examples = (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            Text::new("text\nlines"),
                            TextBackgroundColor::from(PURPLE),
                            TextShadow::default(),
                            Node {
                                border,
                                margin: UiRect::all(Val::Px(20.)),
                                padding: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                overflow: Overflow::scroll(),
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let image = asset_server.load("branding/icon.png");
    let image_borders_examples = (
        Name::new("Image Borders Example"),
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            move |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            ImageNode::new(image.clone()),
                            Node {
                                border,
                                width: Val::Px(80.),
                                // height: Val::Px(60.),
                                margin: UiRect::all(Val::Px(20.)),
                                padding: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    let image = asset_server.load("branding/icon.png");
    let image_content_box_borders_examples = (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            align_self: AlignSelf::Stretch,
            justify_self: JustifySelf::Stretch,
            flex_wrap: FlexWrap::Wrap,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            align_content: AlignContent::FlexStart,
            ..default()
        },
        Children::spawn(SpawnIter(border_labels.into_iter().zip(borders).map(
            move |(label, border)| {
                (
                    Node {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    children![
                        (
                            ImageNode::new(image.clone()),
                            Node {
                                border,
                                width: Val::Px(80.),
                                // height: Val::Px(60.),
                                margin: UiRect::all(Val::Px(20.)),
                                padding: UiRect::all(Val::Px(20.)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                box_sizing: BoxSizing::ContentBox,
                                ..default()
                            },
                            BackgroundColor(MAROON.into()),
                            BorderColor(RED.into()),
                            Outline {
                                width: Val::Px(6.),
                                offset: Val::Px(0.),
                                color: Color::WHITE,
                            },
                        ),
                        (Text::new(label), TextFont::from_font_size(9.0))
                    ],
                )
            },
        ))),
    );

    commands.spawn((
        Name::new("Root"),
        Node {
            width: Val::Vw(50.0),
            height: Val::Vh(50.0),
            margin: UiRect::all(Val::Auto),
            flex_direction: FlexDirection::Column,
            align_self: AlignSelf::Stretch,
            overflow: Overflow::scroll_y(), // n.b.
            ..default()
        },
        // Node {
        //     display: Display::Block,
        //     width: Val::Vw(95.0),
        //     height: Val::Vh(95.0),
        //     margin: UiRect::all(Val::Auto),
        //     overflow: Overflow {
        //         x: OverflowAxis::Hidden,
        //         y: OverflowAxis::Scroll,
        //     },
        //     ..default()
        // },
        BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        children![
            label("Borders"),
            borders_examples,
            label("Borders Rounded"),
            borders_examples_rounded,
            label("Empty Borders"),
            empty_borders_examples,
            label("Text Borders"),
            text_borders_examples,
            label("Image Borders"),
            image_borders_examples,
            // text_borders_examples,
            // image_borders_examples,
            // image_content_box_borders_examples,
            // (
            //     Name::new("Content"),
            //     Node {
            //         display: Display::Block,
            //         width: Val::Percent(100.0),
            //         ..default()
            //     },
            //     // Node {
            //     //     width: Val::Percent(100.0),
            //     //     display: Display::Flex,
            //     //     flex_direction: FlexDirection::Column,
            //     //     align_self: AlignSelf::Stretch,
            //     //     flex_wrap: FlexWrap::Wrap,
            //     //     justify_content: JustifyContent::FlexStart,
            //     //     align_items: AlignItems::FlexStart,
            //     //     align_content: AlignContent::FlexStart,
            //     //     ..default()
            //     // },
            //     children![
            //         label("Borders"),
            //         borders_examples,
            //         label("Borders Rounded"),
            //         borders_examples_rounded,
            //         label("Empty Borders"),
            //         empty_borders_examples,
            //         label("Text Borders"),
            //         text_borders_examples,
            //         label("Image Borders"),
            //         image_borders_examples,
            //     ]
            // )
        ],
    ));
}

// A label widget that accepts a &str and returns
// a Bundle that can be spawned
fn label(text: &str) -> impl Bundle {
    (
        Node {
            margin: UiRect::all(Val::Px(25.0)),
            ..default()
        },
        Outline {
            width: Val::Px(6.),
            offset: Val::Px(0.),
            color: Color::WHITE,
        },
        children![(Text::new(text), TextFont::from_font_size(20.0))],
    )
}
