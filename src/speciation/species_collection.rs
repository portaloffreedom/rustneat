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

use std::cmp::Ordering;

use crate::speciation::{Individual, Species};
use crate::speciation;
use std::slice::{Iter, IterMut};

pub struct SpeciesCollection<I: Individual<F>, F: num::Float> {
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

    pub fn len(&self) -> usize {
        self.collection.len()
    }

    pub fn push(&mut self, species: Species<I, F>) {
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

    /// Iterates through the species
    pub fn iter(&self) -> Iter<'_, Species<I, F>> { self.collection.iter() }

    /// Iterates through the (mutable) species
    pub fn iter_mut(&mut self) -> IterMut<'_, Species<I, F>> { self.collection.iter_mut() }

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
    pub fn compute_update(&mut self) {
        // The old best species will be invalid at the first iteration
        let old_best = self.get_best();

        for species in &mut self.collection {
            species.increase_generations();
            // This value increases continuosly and it's reset every time a better fitness is found
            species.increase_no_improvements_generations();
        }

        // If the old_fitness is valid,
        // it's our best species and we should keep it artifically YOUNG
        if let Some(old_best) = old_best {
            self.collection[old_best].reset_age();
        }
    }

    /// Returns the index pointing to the best species.
    pub fn get_best(&mut self) -> Option<usize> {
        assert!(!self.collection.is_empty());
        if self.cache_need_updating {
            self._update_cache();
        }

        self.best
    }

    /**
     * Finds the worst species (based on the best fitness of that species)
     * Crashes if there are no species with at least `minimal_size` individuals
     *
     * This function is not const because it returns a modifiable iterator.
     *
     * @param minimal_size Species with less individuals than this will not be considered
     * @param exclude_id_list Species in this list will be ignored
     * @return the iterator pointing to the worst species
     */
    pub fn get_worst(&self) -> Option<usize> {
        todo!()
    }

    /// Calculates the number of individuals inside all species
    /// WARNING! The values is not cached and is recalculated every time.
    pub fn count_individuals(&self) -> usize {
        self.collection.iter()
            .map(|species| species.len())
            .sum()
    }

    /// Updates the cached values
    /// WARNING: Cannot cache Worst value, because it's value depends on other parameters (minimal size and others)
    fn _update_cache(&mut self) {
        assert!(!self.collection.is_empty());

        // Best
        self.best = self.collection.iter()
            .enumerate()
            .filter_map(|(i, species)| {
                // if best_fitness is None, this species will be filtered out
                species.get_best_fitness().map(|f| (i, f))
            })
            .max_by(|(_, fitness_a), (_, fitness_b)| if fitness_a > fitness_b { Ordering::Greater } else { Ordering::Less })
            .map(|(i, _)| i);

        // Cannot calculate WORST cache, because there are 2 different
        // version of the worst individual. Which one should be cached?

        self.cache_need_updating = false;
    }
}