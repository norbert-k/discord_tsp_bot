use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use clap::Parser;
use serenity::Client;
use serenity::prelude::{GatewayIntents, TypeMapKey};
use songbird::input::cached::{Compressed, Memory};
use songbird::input::Input;
use songbird::SerenityInit;
use crate::cli::CliArguments;

mod cli;
mod handling;
mod events;
mod actions;
mod sound;

enum CachedSound {
    Compressed(Compressed),
    Uncompressed(Memory),
}

impl From<&CachedSound> for Input {
    fn from(obj: &CachedSound) -> Self {
        use CachedSound::*;
        match obj {
            Compressed(c) => c.new_handle()
                .into(),
            Uncompressed(u) => u.new_handle()
                .try_into()
                .expect("Failed to create decoder for Memory source."),
        }
    }
}

struct SoundStore;

impl TypeMapKey for SoundStore {
    type Value = Arc<Mutex<HashMap<String, CachedSound>>>;
}


#[tokio::main]
async fn main() -> handling::Result<()> {
    color_eyre::install()?;
    let args = CliArguments::parse();
    let intents = GatewayIntents::GUILD_VOICE_STATES;
    let mut discord_client = Client::builder(args.token, intents)
        .event_handler(events::handler::Handler)
        .register_songbird()
        .await?;

    {
        let mut data = discord_client.data.write().await;

        let mut audio_map = HashMap::new();

        let ting_src = Memory::new(songbird::input::ffmpeg("intro.mp3").await.expect("File not found")).unwrap();
        let _ = ting_src.raw.spawn_loader();
        audio_map.insert("ting".into(), CachedSound::Uncompressed(ting_src));

        data.insert::<SoundStore>(Arc::new(Mutex::new(audio_map)))
    }

    if let Err(error) = discord_client.start().await {
        println!("Failed to start: {:?}", error)
    }

    Ok(())
}
