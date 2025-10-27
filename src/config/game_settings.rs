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

    pub fn print_statistics(&self) {
        let total_width: usize = 60;
        let inner_width = total_width - 2;

        // Collect rows as (left_label, right_string)
        let mut rows: Vec<(String, String)> = Vec::new();

        // General
        rows.push((
            "Vehicles reached destination:".to_string(),
            format!("{}", self.statistics.vehicles_passed),
        ));

        // Spawned header as a full-line row (right empty)
        rows.push(("Vehicles spawned:".to_string(), "".to_string()));
        for ((veh_type, color), count) in &self.statistics.vehicles_spawned {
            let left = format!("- {} {}", veh_type, color);
            rows.push((left, format!("{}", count)));
        }

        // Speed
        rows.push((
            "Max velocity recorded:".to_string(),
            format!("{}", self.statistics.max_velocity_reached),
        ));
        rows.push((
            "Min velocity recorded:".to_string(),
            format!(
                "{}",
                if self.statistics.min_velocity_reached == i32::MAX {
                    0
                } else {
                    self.statistics.min_velocity_reached
                }
            ),
        ));

        // Time (include unit in right string)
        let max_t = self.statistics.max_time_in_intersection;
        let min_t = if self.statistics.min_time_in_intersection == f32::MAX {
            0.0
        } else {
            self.statistics.min_time_in_intersection
        };
        rows.push((
            "Max time in intersection:".to_string(),
            format!("{:.1} s", max_t),
        ));
        rows.push((
            "Min time in intersection:".to_string(),
            format!("{:.1} s", min_t),
        ));

        // Other
        rows.push((
            "Close calls detected:".to_string(),
            format!("{}", self.statistics.close_calls),
        ));

        // Compute the width needed by the right column (largest right_string)
        let right_width = rows.iter().map(|(_, r)| r.len()).max().unwrap_or(0);

        // Minimal spacing: we want at least 1 space between left and right columns.
        let min_gap = 1usize;

        // Compute how much space is available for the left column.
        // We reserve: right_width + min_gap for the right side, remaining (inner_width - right_width - min_gap) for left.
        let left_width = if inner_width > right_width + min_gap {
            inner_width - right_width - min_gap
        } else {
            // Fallback: if numbers are too wide for inner_width, reduce right_width by truncating later.
            0
        };

        // Helpers
        let top_border = format!("╔{}╗", "═".repeat(inner_width + 1));
        let sep_border = format!("╟{}╢", "─".repeat(inner_width + 1));
        let bottom_border = format!("╚{}╝", "═".repeat(inner_width + 1));

        fn truncate_to(s: &str, max_len: usize) -> String {
            if s.chars().count() <= max_len {
                return s.to_string();
            }
            // If we must truncate, leave room for "…" char
            if max_len == 0 {
                return "".to_string();
            }
            let mut truncated = String::new();
            for c in s.chars().take(max_len - 1) {
                truncated.push(c);
            }
            truncated.push('…');
            truncated
        }

        // Print box
        println!();
        println!("{}", top_border);
        println!("║{:^inner_width$}║", " SIMULATION STATISTICS 🚦");
        println!("{}", sep_border);

        // Print rows
        for (left, right) in rows {
            // If left_width is zero (extremely small inner_width), we show only the right aligned.
            if left_width == 0 {
                // If right too wide, truncate it
                let right_print = if right.len() > inner_width {
                    truncate_to(&right, inner_width)
                } else {
                    right.clone()
                };
                // center left empty and right aligned to the inner width
                println!("║{:>inner_width$}║", right_print);
                continue;
            }

            // For entries where right is empty (like "Vehicles spawned:"), print the left spanning the full left area
            if right.is_empty() {
                let left_print = if left.chars().count() > left_width {
                    truncate_to(&left, left_width)
                } else {
                    left.clone()
                };
                println!(
                    "║ {:<left_w$} {:>right_w$} ║",
                    left_print,
                    "",                      // empty right
                    left_w = left_width - 1, // we already used 1 space after '║'
                    right_w = right_width
                );
                continue;
            }

            // Normal case: print left truncated/padded, then gap, then right right-aligned
            let left_print = if left.chars().count() > left_width {
                truncate_to(&left, left_width)
            } else {
                left.clone()
            };

            // If right string too long to fit right_width (shouldn't happen because right_width computed), truncate defensively
            let right_print = if right.len() > right_width {
                truncate_to(&right, right_width)
            } else {
                right.clone()
            };

            // Format: "║ " + left_print padded to left_width-1 + " " gap + right_print padded to right_width + " ║"
            // We subtract 1 from left padding because we're explicitly printing a leading single space after '║' for breathing room.
            println!(
                "║ {:<left_w$} {:>right_w$} ║",
                left_print,
                right_print,
                left_w = left_width - 1,
                right_w = right_width
            );
        }

        println!("{}", bottom_border);
        println!();
    }
}
