/* 
 * This file is part of the rustneat project.
 * Copyright (c) 2021 Matteo De Carlo.
 * 
 * This program is free software: you can redistribute it and/or modify  
 * it under the terms of the GNU General Public License as published by  
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but 
 * WITHOUT ANY WARRANTY; without even the implied warranty of 
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU 
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License 
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

pub struct Conf {
    /// Total population size
    pub total_population_size: usize,
    /// If to enable crossover
    pub crossover: bool,

    // SPECIES specific parameters

    /// when to consider a species young (inclusive)
    pub young_age_threshold: usize,
    /// when to consider a species old (inclusive)
    pub old_age_threshold: usize,
    /// when to consider a species stagnating (inclusive)
    pub species_max_stagnation: usize,

    /// multiplier for the fitness of young species (keep > 1)
    pub young_age_fitness_boost: f64,
    /// multiplier for the fitness of old species (keep > 0 and < 1)
    pub old_age_fitness_penalty: f64,
}

impl Conf {
    pub fn new(
        total_population_size: usize,
        crossover: bool,
        young_age_threshold: usize,
        old_age_threshold: usize,
        species_max_stagnation: usize,
        young_age_fitness_boost: f64,
        old_age_fitness_penalty: f64,
    ) -> Self {
        Self {
            total_population_size,
            crossover,
            young_age_threshold,
            old_age_threshold,
            species_max_stagnation,
            young_age_fitness_boost,
            old_age_fitness_penalty,
        }
    }
}

impl Default for Conf {
    fn default() -> Self {
        Self {
            total_population_size: 100,
            crossover: true,
            young_age_threshold: 10,
            old_age_threshold: 40,
            species_max_stagnation: 400,
            young_age_fitness_boost: 1.1,
            old_age_fitness_penalty: 0.9,
        }
    }
}