use super::AppState;
use bevy::{ prelude::*, app::AppExit };

pub struct MainMenuPlugin;

struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

struct MenuColors {
    root: Color,
    border: Color,
    menu: Color,
    button: Color,
    button_hovered: Color,
    button_pressed: Color,
    button_text: Color,
}

impl FromWorld for MenuColors {
    fn from_world(world: &mut World) -> Self {
        MenuColors {
            root: Color::NONE,
            border: Color::RED,
            menu: Color::BLACK,
            button: Color::BLUE,
            button_hovered: Color::rgb(0.25, 0.25, 0.25),
            button_pressed: Color::rgb(0.35, 0.75, 0.35),
            button_text: Color::WHITE,
        }
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Quit,
}

// fn button_system(
//     materials: Res<MenuColors>,
//     mut buttons: Query<&Interaction, (Changed<Interaction>, With<Button>)>
// ) {
//     for (interaction, mut material) in buttons.iter_mut() {
//         match *interaction {
//             Interaction::Clicked => {
//                 *material = materials.button_pressed.clone();
//             }
//             Interaction::Hovered => {
//                 *material = materials.button_hovered.clone();
//             }
//             Interaction::None => {
//                 *material = materials.button.clone();
//             }
//         }
//     }
// }

fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<AppState>>,
    mut exit: EventWriter<AppExit>
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play =>
                    state.set(AppState::InGame).expect("Couldn't switch state to InGame"),
                MenuButton::Quit => exit.send(AppExit),
            }
        }
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuColors>()
            //.add_system(button_system)
            .add_system(button_press_system)
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup));
    }
}

fn root(materials: &Res<MenuColors>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: UiColor(materials.root),
        ..Default::default()
    }
}

fn border(materials: &Res<MenuColors>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(95.0), Val::Percent(95.0)),
            border: UiRect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: UiColor(materials.border),
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuColors>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(Val::Percent(5.0)),
            ..Default::default()
        },
        color: UiColor(materials.menu),
        ..Default::default()
    }
}

fn button(materials: &Res<MenuColors>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(33.0), Val::Percent(33.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Percent(5.0)),
            ..Default::default()
        },
        color: UiColor(materials.button),
        ..Default::default()
    }
}

fn button_text(
    asset_server: &Res<AssetServer>,
    materials: &Res<MenuColors>,
    label: &str
) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: UiRect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::from_section(label, TextStyle {
            font_size: 30.0,
            color: materials.button_text,
            ..Default::default()
        }),
        ..Default::default()
    };
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: Res<MenuColors>) {
    let camera_entity = commands.spawn_bundle(Camera2dBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent.spawn_bundle(border(&materials)).with_children(|parent| {
                // left vertical fill (content)
                parent.spawn_bundle(menu_background(&materials)).with_children(|parent| {
                    parent
                        .spawn_bundle(button(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button_text(&asset_server, &materials, "New Game"));
                        })
                        .insert(MenuButton::Play);
                    parent
                        .spawn_bundle(button(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button_text(&asset_server, &materials, "Quit"));
                        })
                        .insert(MenuButton::Quit);
                });
            });
        })
        .id();

    commands.insert_resource(MainMenuData {
        camera_entity,
        ui_root,
    });
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}
