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
use crate::speciation::{Species, Individual};

struct SpeciesCollection<I: Individual<F>, F: num::Float> {
    collection: Vec<Species<I, F>>,
    best: Option<usize>,
    cache_need_updating: bool,
}

impl<I: Individual<F>, F: num::Float> SpeciesCollection<I, F> {
    pub fn new<It: Iterator<Item=Species<I, F>>>(species: It) -> Self {
        Self {
            collection: species.into_iter().collect(),
            best: None,
            cache_need_updating: true,
        }
    }
}

pub struct Genus<I: Individual<F>, F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<I, F>,
}