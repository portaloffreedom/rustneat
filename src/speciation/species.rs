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

use crate::speciation::{Individual, Age, Conf};
use std::cmp::Ordering;
use std::process::Output;
use std::ops::Div;

// #[derive(Clone)]
struct Indiv<F> {
    adjusted_fitness: Option<F>,
    individual: Box<dyn Individual<F>>,
}

impl<F> From<Box<dyn Individual<F>>> for Indiv<F> {
    fn from(individual: Box<dyn Individual<F>>) -> Self {
        Indiv {
            individual,
            adjusted_fitness: None,
        }
    }
}

pub struct Species<F: Default + Copy + PartialOrd + Div<f64, Output=F>> {
    individuals: Vec<Indiv<F>>,
    id: usize,
    age: Age,
    last_best_fitness: F,
}

impl<F: Default + Copy + PartialOrd + Div<f64, Output=F>> Species<F> {
    pub fn new(individual: Box<dyn Individual<F>>, species_id: usize) -> Self {
        Self {
            individuals: vec![Indiv::from(individual)],
            id: species_id,
            age: Age::new(),
            last_best_fitness: F::default(),
        }
    }

    pub fn clone_with_new_individuals<I>(&self, new_individuals: I) -> Self
        where I: Iterator<Item=Box<dyn Individual<F>>> {
        Self {
            individuals: new_individuals.map(|i| Indiv::from(i)).collect(),
            id: self.id,
            age: self.age.clone(),
            last_best_fitness: self.last_best_fitness.clone(),
        }
    }

    pub fn is_compatible(&self, candidate: &dyn Individual<F>) -> bool {
        if let Some(representative) = self.representative() {
            representative.is_compatible(candidate)
        } else {
            false
        }
    }

    pub fn get_best_individual(&self) -> Option<&Box<dyn Individual<F>>> {
        self.individuals.iter()
            .map(|i| &i.individual)
            .max_by(|a, b| if a.fitness() > b.fitness() { Ordering::Greater } else { Ordering::Less })
    }

    pub fn get_best_fitness(&self) -> Option<F> {
        self.get_best_individual()
            .map(|i| i.fitness())
            .flatten()
    }

    /// This method performs fitness sharing. It computes the adjusted fitness of the individuals.
    /// It also boosts the fitness of the young and penalizes old species.
    ///
    /// # Arguments
    ///
    /// * `is_best_species` set to true if this is the best species
    ///
    pub fn compute_adjust_fitness(&mut self, is_best_species: bool, conf: &Conf) {
        assert!(!self.is_empty());

        let individual_n = self.individuals.len();

        // Iterates through individuals and sets the adjusted fitness
        self.individuals.iter_mut()
            .for_each(|indiv| {
                let fitness = indiv.individual.fitness().unwrap_or_default();

                if fitness < F::default() {
                    panic!("FITNESS CANNOT BE NEGATIVE");
                }
                let f_adj: F = self.individual_adjusted_fitness(fitness, is_best_species, conf);

                // Compute the adjusted fitness for this member
                indiv.adjusted_fitness = Some(f_adj / individual_n as f64);
            });
    }

    /// Inserts an individual into this species
    pub fn insert(&mut self, individual: Box<dyn Individual<F>>) {
        self.individuals.push(Indiv::from(individual))
    }

    /// Replaces set of individuals with a new set of individuals
    pub fn set_individuals<It: Iterator<Item=Box<dyn Individual<F>>>>(&mut self, iterator: It) {
        self.individuals.clear();
        self.individuals = iterator.into_iter()
            .map(|i| Indiv::from(i))
            .collect()
    }

    pub fn iter() { unimplemented!() }
    pub fn iter_mut() { unimplemented!() }

    pub fn is_empty(&self) -> bool {
        self.individuals.is_empty()
    }

    pub fn len(&self) -> usize { self.individuals.len() }

    pub fn representative<'a>(&'a self) -> Option<&'a Box<dyn Individual<F>>> {
        self.individuals.first().map(|i| &i.individual)
    }

    fn individual_adjusted_fitness(&mut self, mut fitness: F, is_best_species: bool, conf: Conf) -> F {
        // set small fitness if it is absent
        if fitness == F::default() {
            fitness = F::from(0.0001);
        }

        // update the best fitness and stagnation counter
        if fitness >= self.last_best_fitness {
            self.last_best_fitness = fitness;
            self.age.reset_no_improvements();
        }

        let number_of_generations = self.age.generations;

        // boost the fitness up to some young age
        if number_of_generations < conf.young_age_threshold {
            fitness *= F::from(conf.young_age_fitness_boost);
        }

        // penalty for old species
        if number_of_generations > conf.old_age_threshold {
            fitness *= F::from(conf.old_age_fitness_penalty);
        }

        // Extreme penalty if this species is stagnating for too long time
        // one exception if this is the best species found so far
        if !is_best_species && self.age.no_improvements > conf.species_max_stagnation {
            fitness *= F::from(0.0000001);
        }

        fitness
    }
}
