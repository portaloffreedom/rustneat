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
use std::borrow::BorrowMut;

use crate::speciation;
use crate::speciation::Individual;

use super::species_collection::SpeciesCollection;

pub struct Genus<I: Individual<F>, F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<I, F>,
}

impl<I: Individual<F>, F: num::Float> Genus<I, F> {
    /// Creates a new Genus object
    pub fn new() -> Self {
        Self {
            next_species_id: 1,
            species_collection: SpeciesCollection::new(),
        }
    }
}