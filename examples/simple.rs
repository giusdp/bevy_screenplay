use bevy::{asset::LoadState, prelude::*};
use bevy_talks::prelude::*;

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    #[default]
    LoadAssets,
    Loaded,
}

#[derive(Resource)]
struct PrintEnabled(bool);

#[derive(Resource)]
struct SimpleScreenplayAsset {
    handle: Handle<RawScreenplay>,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TalksPlugin))
        .add_state::<AppState>()
        .insert_resource(PrintEnabled(true))
        .add_systems(
            OnEnter(AppState::LoadAssets),
            load_talks.run_if(in_state(AppState::LoadAssets)),
        )
        .add_systems(Update, check_loading.run_if(in_state(AppState::LoadAssets)))
        .add_systems(OnEnter(AppState::Loaded), setup_screenplay)
        .add_systems(Update, (interact, print, bevy::window::close_on_esc))
        .run();
}

fn load_talks(mut commands: Commands, server: Res<AssetServer>) {
    let h: Handle<RawScreenplay> = server.load("simple.json");
    commands.insert_resource(SimpleScreenplayAsset { handle: h });
}

fn check_loading(
    server: Res<AssetServer>,
    simple_sp_asset: Res<SimpleScreenplayAsset>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let load_state = server.get_load_state(&simple_sp_asset.handle);
    if load_state == LoadState::Loaded {
        next_state.set(AppState::Loaded);
    }
}

fn setup_screenplay(
    mut commands: Commands,
    raws: Res<Assets<RawScreenplay>>,
    simple_sp_asset: Res<SimpleScreenplayAsset>,
) {
    let screenplay = ScreenplayBuilder::new()
        .with_raw_screenplay(simple_sp_asset.handle.clone())
        .build(&raws)
        .unwrap();
    commands.spawn(screenplay);

    println!("Press space to advance the conversation.");
}

fn print(mut print_enabled: ResMut<PrintEnabled>, sp_query: Query<&Screenplay>) {
    if !print_enabled.0 {
        return;
    }

    for sp in &sp_query {
        // extract actors names into a vector
        let actors = sp
            .actors()
            .iter()
            .map(|a| a.name.to_owned())
            .collect::<Vec<String>>();

        let mut speaker = "Narrator";
        if actors.len() > 0 {
            speaker = actors[0].as_str();
        }

        match sp.action_kind() {
            ActionKind::Talk => println!("{}: {}", speaker, sp.text()),
            ActionKind::Enter => println!("--- {actors:?} enters the scene."),
            ActionKind::Exit => println!("--- {actors:?} exit the scene."),
            ActionKind::Choice => todo!(),
        };
        print_enabled.0 = false;
    }
}

fn interact(
    input: Res<Input<KeyCode>>,
    mut sp_query: Query<&mut Screenplay>,
    mut print_enabled: ResMut<PrintEnabled>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut sp in &mut sp_query {
            match sp.next_action() {
                Ok(_) => print_enabled.0 = true,
                Err(e) => error!("Error: {:?}", e),
            }
        }
    }
}
