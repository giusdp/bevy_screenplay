// use bevy::prelude::*;
// use bevy_talks::prelude::*;

// #[derive(Resource)]
// struct ScreenplayHandle(Handle<Screenplay>);

// #[derive(Resource)]
// struct PrintEnabled(bool);

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(TalksPlugin)
//         .insert_resource(PrintEnabled(true))
//         .add_startup_system(setup)
//         .add_system(interact)
//         .add_system(print)
//         .add_system(bevy::window::close_on_esc)
//         .run();
// }

// fn setup(mut commands: Commands, server: Res<AssetServer>) {
//     let handle: Handle<Screenplay> = server.load("full.json");
//     commands.insert_resource(ScreenplayHandle(handle));

//     println!("Press space to advance the conversation.");
// }

// fn print(
//     mut print_enabled: ResMut<PrintEnabled>,
//     screenplays: Res<Assets<Screenplay>>,
//     sp_handle: Res<ScreenplayHandle>,
// ) {
//     if !print_enabled.0 {
//         return;
//     }
//     let screenplay = screenplays.get(&sp_handle.0).unwrap();

//     let actors = screenplay
//         .actors()
//         .map(|a| a.iter().map(|a| a.name.to_owned()).collect::<Vec<String>>())
//         .and_then(|names| {
//             if names.is_empty() {
//                 Some("Narrator".to_string())
//             } else {
//                 Some(names.join(" and "))
//             }
//         });

//     match screenplay.action_kind() {
//         ActionKind::PlayerChoice => {
//             println!("Choices:");
//             for (i, choice) in screenplay.choices().unwrap().iter().enumerate() {
//                 println!("{}: {}", i + 1, choice.text);
//             }
//         }
//         ActionKind::ActorTalk => println!("{}: {}", actors.unwrap(), screenplay.text()),
//         ActionKind::ActorEnter => println!("--- {} enters the scene.", actors.unwrap()),
//         ActionKind::ActorExit => println!("--- {} exit the scene.", actors.unwrap()),
//     };

//     print_enabled.0 = false;
// }

// fn interact(
//     input: Res<Input<KeyCode>>,
//     sp_handle: ResMut<ScreenplayHandle>,
//     mut screenplays: ResMut<Assets<Screenplay>>,
//     mut print_enabled: ResMut<PrintEnabled>,
// ) {
//     let screenplay = screenplays.get_mut(&sp_handle.0).unwrap();

//     if screenplay.at_player_action() {
//         if input.just_pressed(KeyCode::Key1) {
//             choose(
//                 screenplay,
//                 screenplay.choices().unwrap()[0].next,
//                 &mut print_enabled,
//             );
//         } else if input.just_pressed(KeyCode::Key2) {
//             choose(
//                 screenplay,
//                 screenplay.choices().unwrap()[1].next,
//                 &mut print_enabled,
//             );
//         }
//     } else if input.just_pressed(KeyCode::Space) {
//         match screenplay.next_action() {
//             Ok(_) => print_enabled.0 = true,
//             Err(e) => {
//                 println!("Error: {:?}", e);
//             }
//         }
//     }
// }

// fn choose(sp: &mut Screenplay, id: ActionId, print: &mut PrintEnabled) {
//     match sp.jump_to(id) {
//         Ok(_) => print.0 = true,
//         Err(e) => {
//             println!("Error: {:?}", e);
//         }
//     }
// }
