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
use crate::speciation::{Individual, Species};
use crate::speciation;
use std::borrow::BorrowMut;

struct SpeciesCollection<I: Individual<F>, F: num::Float> {
    collection: Vec<Species<I, F>>,
    best: Option<usize>,
    cache_need_updating: bool,
}

impl<I: Individual<F>, F: num::Float> SpeciesCollection<I, F> {
    pub fn new() -> Self {
        Self {
            collection: Vec::new(),
            best: None,
            cache_need_updating: true,
        }
    }

    pub fn new_from_iter<It: Iterator<Item=Species<I, F>>>(species: It) -> Self {
        Self {
            collection: species.into_iter().collect(),
            best: None,
            cache_need_updating: true,
        }
    }

    pub fn push(&mut self, species: Species<I,F>) {
        self.collection.push(species);
        self.cache_need_updating = true;
    }

    /// Removes all empty species (cleanup routine for every case..)
    pub fn cleanup(&mut self) {
        self.collection.retain(|species| !species.is_empty());
    }

    /// Deletes all species
    pub fn clear(&mut self) {
        self.collection.clear()
    }

    /// Computes the adjusted fitness for all species
    pub fn compute_adjust_fitness(&mut self, conf: &speciation::Conf)
    {
        let best = self.best.expect("best should be present");
        let best_id = self.collection[best].id;
        for species in &mut self.collection {
            species.compute_adjust_fitness(species.id == best_id, conf);
        }
    }

    /// Updates the best_species, increases age for all species
    ///
    /// The best species gets through a rejuvenating process
    pub fn compute_fitness(&mut self) {
        // The old best species will be invalid at the first iteration
        let old_best = self.get_best();

        todo!()
    }
}

pub struct Genus<I: Individual<F>, F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<I, F>,
}

impl<I: Individual<F>, F: num::Float> Genus<I,F> {
    /// Creates a new Genus object
    pub fn new() -> Self {
        Self {
            next_species_id: 1,
            species_collection: SpeciesCollection::new(),
        }
    }
}