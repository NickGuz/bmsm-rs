use crate::consts::*;
use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct ScoreResource {
    corrects: usize,
    fails: usize,
    score: usize,
    pub pgreats: usize,
    pub greats: usize,
    pub goods: usize,
    pub bads: usize,
    pub poors: usize,
}

impl ScoreResource {
    /// Increases number of corrects and adds to score
    pub fn increase_correct(&mut self, distance: f32) -> usize {
        self.corrects += 1;

        // Get a value from 0 to 1 according to how close the press was
        let score_multiplier = (THRESHOLD - distance.abs()) / THRESHOLD;

        // Give at least 10 points and 100 at max
        let points = (score_multiplier * 100.).min(100.).max(10.) as usize;
        self.score += points;

        points
    }

    /// Increases number of failures
    pub fn increase_fails(&mut self) {
        self.fails += 1;
    }

    pub fn reset(&mut self) {
        self.corrects = 0;
        self.fails = 0;
        self.score = 0;
        self.pgreats = 0;
        self.greats = 0;
        self.goods = 0;
        self.bads = 0;
        self.poors = 0;
    }

    // Getters -- this seems stupid but i'm just following the tutorial for now
    pub fn score(&self) -> usize {
        self.score
    }
    pub fn corrects(&self) -> usize {
        self.corrects
    }
    pub fn fails(&self) -> usize {
        self.fails
    }
}
