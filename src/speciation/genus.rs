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
use std::ops::Div;

struct SpeciesCollection<F: Default + Copy + PartialOrd + Div<f64, Output = F>> {
    collection: Vec<Species<F>>,
    best: usize,
    cache_need_updating: bool,
}

pub struct Genus<F: Default + Copy + PartialOrd + Div<f64, Output = F>> {
    next_species_id: usize,
    species_collection: SpeciesCollection<F>,
}