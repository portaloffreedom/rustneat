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

use std::fmt::Debug;
use crate::speciation::Individual;
use num::Float;
use crate::speciation::species_collection::SpeciesCollection;

pub struct GenusSeed<'individuals, I: Individual<F>, F: Float> {
    orphans: Vec<I>,
    new_species_collection: SpeciesCollection<I,F>,
    need_evaluation: Vec<&'individuals mut I>,
    old_species_individuals: Vec<Vec<&'individuals I>>
}

impl<'individuals, I: Individual<F>, F: Float+Debug> GenusSeed<'individuals, I,F> {
    pub fn new(
        orphans: Vec<I>,
        new_species_collection: SpeciesCollection<I,F>,
        need_evaluation: Vec<&'individuals mut I>,
        old_species_individuals: Vec<Vec<&'individuals I>>) -> Self {
        Self {
            orphans,
            new_species_collection,
            need_evaluation,
            old_species_individuals,
        }
    }

    pub fn evaluate<E: Fn(&mut I) -> F >(&mut self, evaluate_individual: E) {
        for new_individual in self.need_evaluation.iter_mut() {
            let fitness: F = evaluate_individual(*new_individual);
            let individual_fitness = new_individual.fitness();
            assert!(individual_fitness.is_some());
            assert_eq!(fitness, individual_fitness.unwrap());
        }
    }
}