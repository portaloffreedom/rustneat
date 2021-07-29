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
use crate::speciation::Species;

struct SpeciesCollection<F: num::Float> {
    collection: Vec<Species<F>>,
    best: Option<usize>,
    cache_need_updating: bool,
}

impl<F: num::Float> SpeciesCollection<F> {
    pub fn new<It: Iterator<Item=Species<F>>>(species: It) -> Self {
        Self {
            collection: species.into_iter().collect(),
            best: None,
            cache_need_updating: true,
        }
    }
}

pub struct Genus<F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<F>,
}