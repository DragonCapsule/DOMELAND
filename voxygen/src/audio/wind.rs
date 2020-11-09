//! Handles ambient wind sounds
use crate::{audio::AudioFrontend, scene::Camera};
use client::Client;
use common::{assets, state::State, terrain::BlockKind, vol::ReadVol};
use serde::Deserialize;
use std::time::Instant;
use tracing::warn;

// For if we want wind to vary strength by time of day
//const DAY_START_SECONDS: u32 = 28800; // 8:00
//const DAY_END_SECONDS: u32 = 70200; // 19:30

#[derive(Debug, Default, Deserialize)]
struct WindCollection {
    tracks: Vec<WindItem>,
}

/// Configuration for a single music track in the soundtrack
#[derive(Debug, Deserialize)]
pub struct WindItem {
    path: String,
    /// Length of the track in seconds
    length: f32,
    /* Whether this track should play during day or night
     * timing: Option<DayPeriod>, */
}

///// Allows control over when a track should play based on in-game time of day
//#[derive(Debug, Deserialize, PartialEq)]
//enum DayPeriod {
//    /// 8:00 AM to 7:30 PM
//    Day,
//    /// 7:31 PM to 6:59 AM
//    Night,
//}

pub struct WindMgr {
    soundtrack: WindCollection,
    began_playing: Instant,
    next_track_change: f32,
    volume: f32,
    tree_multiplier: f32,
}

impl WindMgr {
    #[allow(clippy::new_without_default)] // TODO: Pending review in #587
    pub fn new() -> Self {
        Self {
            soundtrack: Self::load_soundtrack_items(),
            began_playing: Instant::now(),
            next_track_change: 0.0,
            volume: 0.0,
            tree_multiplier: 0.0,
        }
    }

    /// Checks whether the previous track has completed. If so, sends a
    /// request to play the next (random) track
    pub fn maintain(
        &mut self,
        audio: &mut AudioFrontend,
        state: &State,
        client: &Client,
        camera: &Camera,
    ) {
        if audio.sfx_enabled() && !self.soundtrack.tracks.is_empty() {
            let player_alt = Self::get_current_alt(client);
            let terrain_alt = Self::get_current_terrain_alt(client);

            let alt_multiplier = (player_alt / 1200.0).abs();

            // Tree density factors into wind volume. The more trees,
            // the less wind
            let mut tree_multiplier = self.tree_multiplier;
            let new_tree_multiplier = if (player_alt - terrain_alt) < 150.0 {
                1.0 - Self::get_current_tree_density(client)
            } else {
                1.0
            };

            // Smooths tree_multiplier transitions between chunks
            if tree_multiplier < new_tree_multiplier {
                tree_multiplier += 0.001;
            } else if tree_multiplier > new_tree_multiplier {
                tree_multiplier -= 0.001;
            }
            self.tree_multiplier = tree_multiplier;

            let mut volume_multiplier = alt_multiplier * self.tree_multiplier;

            let focus_off = camera.get_focus_pos().map(f32::trunc);
            let cam_pos = camera.dependents().cam_pos + focus_off;

            // Checks if the camera is underwater to stop wind sounds
            if state
                .terrain()
                .get((cam_pos).map(|e| e.floor() as i32))
                .map(|b| b.kind())
                .unwrap_or(BlockKind::Air)
                == BlockKind::Water
            {
                volume_multiplier = volume_multiplier * 0.1;
            }
            if cam_pos.z < Self::get_current_terrain_alt(client) - 20.0 {
                volume_multiplier = 0.0;
            }

            if volume_multiplier > 1.0 {
                volume_multiplier = 1.0
            }

            let target_volume = volume_multiplier;

            // Transitions the wind smoothly
            self.volume = audio.get_wind_volume();
            if self.volume < target_volume {
                audio.set_wind_volume(self.volume + 0.001);
            } else if self.volume > target_volume {
                audio.set_wind_volume(self.volume - 0.001);
            }

            if self.began_playing.elapsed().as_secs_f32() > self.next_track_change {
                //let game_time = (state.get_time_of_day() as u64 % 86400) as u32;
                //let current_period_of_day = Self::get_current_day_period(game_time);
                let track = &self.soundtrack.tracks[0];

                self.began_playing = Instant::now();
                self.next_track_change = track.length;

                audio.play_wind(&track.path, volume_multiplier);
            }
        }
    }

    //fn get_current_day_period(game_time: u32) -> DayPeriod {
    //    if game_time > DAY_START_SECONDS && game_time < DAY_END_SECONDS {
    //        DayPeriod::Day
    //    } else {
    //        DayPeriod::Night
    //    }
    //}

    fn get_current_alt(client: &Client) -> f32 {
        match client.current_position() {
            Some(pos) => pos.z,
            None => 0.0,
        }
    }

    fn get_current_terrain_alt(client: &Client) -> f32 {
        if let Some(chunk) = client.current_chunk() {
            chunk.meta().alt()
        } else {
            0.0
        }
    }

    fn get_current_tree_density(client: &Client) -> f32 {
        match client.current_chunk() {
            Some(current_chunk) => current_chunk.meta().tree_density(),
            None => 0.0,
        }
    }

    fn load_soundtrack_items() -> WindCollection {
        match assets::load_file("voxygen.audio.wind", &["ron"]) {
            Ok(file) => match ron::de::from_reader(file) {
                Ok(config) => config,
                Err(error) => {
                    warn!(
                        "Error parsing music config file, music will not be available: {}",
                        format!("{:#?}", error)
                    );

                    WindCollection::default()
                },
            },
            Err(error) => {
                warn!(
                    "Error reading music config file, music will not be available: {}",
                    format!("{:#?}", error)
                );

                WindCollection::default()
            },
        }
    }
}
