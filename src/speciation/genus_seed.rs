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
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;
use crate::speciation::Individual;
use num::Float;
use crate::speciation::species::RcSpecies;
use crate::speciation::species_collection::SpeciesCollection;

pub struct GenusSeed<I: Individual<F>, F: Float> {
    pub orphans: Vec<Rc<RefCell<I>>>,
    pub new_species_collection: Vec<RcSpecies<I,F>>,
    pub need_evaluation: Vec<Rc<RefCell<I>>>,
    pub old_species_individuals: Vec<Vec<I>>
}

impl<I: Individual<F>, F: Float+Debug> GenusSeed<I,F> {
    pub fn new(
        orphans: Vec<Rc<RefCell<I>>>,
        new_species_collection: Vec<RcSpecies<I,F>>,
        need_evaluation: Vec<Rc<RefCell<I>>>,
        old_species_individuals: Vec<Vec<I>>) -> Self {
        Self {
            orphans,
            new_species_collection,
            need_evaluation,
            old_species_individuals,
        }
    }

    pub fn evaluate<E: FnMut(&mut I) -> F >(&mut self, mut evaluate_individual: E) {
        for mut new_individual in self.need_evaluation.iter_mut() {
            let fitness: F = evaluate_individual(new_individual.as_ref().borrow_mut().borrow_mut());
            let individual_fitness = new_individual.borrow().fitness();
            assert!(individual_fitness.is_some());
            assert_eq!(fitness, individual_fitness.unwrap());
        }
    }
}