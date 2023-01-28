use std::sync::Arc;
use serenity::client::Context;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::prelude::EventHandler;
use async_trait::async_trait;
use serenity::model::prelude::VoiceState;
use songbird::{Event, EventContext, Songbird, TrackEvent};
use songbird::driver::Bitrate::BitsPerSecond;
use songbird::events::EventData;
use songbird::id::GuildId;
use crate::SoundStore;

pub struct Handler;

pub struct SongbirdHandler {
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl songbird::events::EventHandler for SongbirdHandler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        println!("STOPPED PLS");
        self.manager.leave(self.guild_id).await;
        None
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, ready: Ready) {
        println!("{:?}", ready.user)
    }

    async fn voice_state_update(&self, ctx: Context, state: VoiceState) {
        println!("Member Joined: {:?}, channel: {:?}", state.member, state.channel_id);
        if state.member.unwrap().user.bot {
            return;
        }
        if let Some(channel_id) = state.channel_id {
            let manager = songbird::get(&ctx)
                .await
                .expect("Songbird Voice client placed in at the initialisation")
                .clone();

            manager.join(state.guild_id.unwrap(), channel_id).await;
            if let Some(handler_lock) = manager.get(state.guild_id.unwrap()) {
                let mut handler = handler_lock.lock().await;
                handler.stop();
                let source = ctx.data.read().await.get::<SoundStore>().cloned().expect("Sound cache error");
                let sources = source.lock().expect("nuce");
                let source = sources.get("ting").expect("Failed to find");
                let driver = source.into();
                handler.play_source(driver)
                    .add_event(Event::Track(TrackEvent::End), SongbirdHandler {
                        manager,
                        guild_id: GuildId::from(state.guild_id.unwrap()),
                    });
            }
        }
    }

    // async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
    //     todo!()
    // }
}