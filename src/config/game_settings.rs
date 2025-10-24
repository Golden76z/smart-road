impl<'a> GameSettings<'a> {
    pub fn print_statistics(&self) {
        println!("\n===== STATISTIQUES DE LA SIMULATION =====");
        println!(
            "Nombre de véhicules passés : {}",
            self.statistics.vehicles_passed
        );
        println!("Nombre de véhicules spawnés :");
        for ((veh_type, color), count) in &self.statistics.vehicles_spawned {
            println!("  - {} {} : {}", veh_type, color, count);
        }
        println!(
            "Vitesse maximale atteinte : {}",
            self.statistics.max_velocity_reached
        );
        println!(
            "Vitesse minimale atteinte : {}",
            if self.statistics.min_velocity_reached == i32::MAX {
                0
            } else {
                self.statistics.min_velocity_reached
            }
        );
        println!(
            "Temps max dans l'intersection : {:.3} s",
            self.statistics.max_time_in_intersection
        );
        println!(
            "Temps min dans l'intersection : {:.3} s",
            if self.statistics.min_time_in_intersection == f32::MAX {
                0.0
            } else {
                self.statistics.min_time_in_intersection
            }
        );
        println!("Nombre de close calls : {}", self.statistics.close_calls);
        println!("Paires de close calls détectées :");
        for (id1, id2) in &self.statistics.close_call_pairs {
            println!("  - Véhicules {} et {}", id1, id2);
        }
        println!("========================================\n");
    }
}
use sdl2::{Sdl, ttf::Sdl2TtfContext};
use std::time::Instant;

use crate::{
    config::{
        Broadcaster, Controller, KEY_COOLDOWN, SPAWN_COOLDOWN, SpawnManager, TrafficLanes, UiState,
        VELOCITY_COOLDOWN, statistics::Statistics,
    },
    render::renderer::Renderer,
};

pub struct GameSettings<'a> {
    pub spawn_manager: SpawnManager,
    pub ui_state: UiState,
    pub render: Renderer,
    pub controller: Controller,
    pub broadcaster: Broadcaster<'a>,
    pub time_tracker: Instant,
    pub lanes: TrafficLanes,
    pub vehicle_id: i16,
    pub statistics: Statistics,
}

impl<'a> GameSettings<'a> {
    pub fn new(sdl_context: &Sdl, ttf_context: &'a Sdl2TtfContext) -> Self {
        let render = Renderer::new(&sdl_context).expect("Failed to create a renderer");

        GameSettings {
            spawn_manager: SpawnManager::new(SPAWN_COOLDOWN, KEY_COOLDOWN, VELOCITY_COOLDOWN),
            ui_state: UiState::new(),
            render: render,
            controller: Controller::new(),
            broadcaster: Broadcaster::new(ttf_context),
            time_tracker: Instant::now(),
            lanes: TrafficLanes::new(),
            vehicle_id: 0,
            statistics: Statistics::new(),
        }
    }

    // Incrementing the vehicles id's for vehicle id attribution and identification
    pub fn id(&mut self) -> i16 {
        self.vehicle_id += 1;
        self.vehicle_id
    }
}
