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

use crate::speciation::{Conf, Individual, Species};
use crate::speciation::genus_seed::GenusSeed;
use crate::speciation::species::SpeciesIter;

use super::species_collection::SpeciesCollection;

pub struct Genus<I: Individual<F>, F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<I, F>,
}

impl<I: 'static + Individual<F>, F: 'static + num::Float + Debug> Genus<I, F> {
    /// Creates a new Genus object
    pub fn new() -> Self {
        Self {
            next_species_id: 1,
            species_collection: SpeciesCollection::new(),
        }
    }

    pub fn species_count(&self) -> usize {
        self.species_collection.len()
    }

    pub fn count_individuals(&self) -> usize {
        self.species_collection.count_individuals()
    }

    /// Creates the species. It takes a list of individuals and splits them into multiple species,
    /// grouping the compatible individuals together.
    ///
    /// *WARNING! THIS FUNCTION TAKES OWNERSHIP OF THE SOURCE ITERATOR FOR INDIVIDUALS*
    pub fn speciate<It: Iterator<Item=I>>(&mut self, source_population: It) {
        // Clear out the species list
        self.species_collection.clear();

        // NOTE: we are comparing the new generation's genomes to the representative from the previous generation!
        // Any new species that is created is assigned a representative from the new generation.
        'individuals: for individual in source_population {
            // Iterate through
            for species in self.species_collection.iter_mut() {
                if species.is_compatible(&individual) {
                    species.insert(individual);
                    continue 'individuals;
                }
            }
            // No compatible species was found, create a new one
            self.species_collection.push(Species::new(individual, self.next_species_id));
            self.next_species_id += 1;
        }
    }

    pub fn ensure_evaluated_population<E: Fn(&mut I) -> F>(&mut self, evaluate_individual: E)
        where F: Debug
    {
        for species in self.species_collection.iter_mut() {
            for individual in species.iter_mut() {
                let fit: Option<F> = individual.fitness();
                if fit.is_none() {
                    let fitness: F = evaluate_individual(individual);
                    let individual_fitness: Option<F> = individual.fitness();
                    assert!(individual_fitness.is_some());
                    assert_eq!(fitness, individual_fitness.unwrap());
                }
            }
        }
    }

    pub fn update(&mut self, conf: &Conf) -> &mut Self {
        // Update species stagbnation and stuff
        self.species_collection.compute_update();
        // Update adjusted fitnesses
        self.species_collection.compute_adjust_fitness(conf);
        self
    }


    /// Creates the genus for the next generation.
    /// The species are copied over so that `this` Genus is not invalidated.
    ///
    /// @param conf Species configuration object
    /// @param selection function to select 1 parent (can be called even if crossover is enabled, when there is not more
    /// than one parent possible)
    /// @param parent_selection function to select 2 parents (only possibly called if crossover is enabled)
    /// @param reproduce_individual_1 function to crossover and create new individuals from 1 parent
    /// @param crossover_individual_2 function to crossover and create new individuals from 2 parents
    /// @param mutate_individual function that mutates an individual
    /// @param population_management function to create the new population from the old and new individual,
    /// size of the new population is passed in as a parameter. The size can vary a lot from one generation to the next.
    /// @param evaluate_individual function to evaluate new individuals
    /// @return the genus of the next generation
    pub fn generate_new_individuals<'a, 'individual, SelectionF, ParentSelectionF, ReproduceI1F, CrossoverI2F, MutateF>(
        &self,
        conf: &Conf,
        selection: &'static SelectionF,
        parent_selection: &'static ParentSelectionF,
        reproduce_individual_1: &'static ReproduceI1F,
        crossover_individual_2: &'static CrossoverI2F,
        mutate_individual: &'static MutateF,
    ) -> GenusSeed<I, F>
        where
            I: 'individual,
            SelectionF: FnMut(Box<SpeciesIter<I, F>>) -> &'individual I,
            ParentSelectionF: FnMut(Box<SpeciesIter<I, F>>) -> (&'individual I,&'individual I),
            ReproduceI1F: FnMut(&I) -> I,
            CrossoverI2F: FnMut(&I, &I) -> I,
            MutateF: FnMut(&mut I),
    {
        // Calculate offspring amount
        let offspring_amounts: Vec<usize> = self.count_offsprings(conf.total_population_size).expect("count offspring to be successful");

        // Clone Species
        let mut new_species_collection: SpeciesCollection<I, F> = SpeciesCollection::new();
        let mut orphans: Vec<I> = Vec::new();

        // Pointers to values in new_species_collection and orphans
        let mut need_evaluation: Vec<&mut I> = Vec::new();

        // Pointers to current const species_collection
        // std::vector < std::vector <const I* > > old_species_individuals;
        let mut old_species_individuals_vec: Vec<Vec<&I>> = Vec::new();

        //////////////////////////////////////////////
        // GENERATE NEW INDIVIDUALS
        for (species_i, species) in self.species_collection.iter().enumerate() {
            let old_species_individuals: Vec<&I> = species.iter().collect();
            old_species_individuals_vec.push(old_species_individuals);

            let mut new_individuals: Vec<I> = Vec::new();
            trait IteratorTrait: ExactSizeIterator {}
            // for (unsigned int n_offspring = 0; n_offspring < offspring_amounts[species_i]; n_offspring+ +)
            for n_offspring in 0_usize..offspring_amounts[species_i] {
                let new_individual: I = self.generate_new_individual::<
                    SpeciesIter<'a, I,F>,
                    SelectionF,
                    ParentSelectionF,
                    ReproduceI1F,
                    CrossoverI2F,
                    MutateF>
                (
                    conf,
                    species.iter(),
                    selection,
                    parent_selection,
                    reproduce_individual_1,
                    crossover_individual_2,
                    mutate_individual,
                );

                // if the new individual is compatible with the species, otherwise create new.
                if species.is_compatible(&new_individual) {
                    new_individuals.push(new_individual);
                    need_evaluation.push(new_individuals.last_mut().unwrap());
                } else {
                    orphans.push(new_individual);
                    need_evaluation.push(orphans.last_mut().unwrap());
                }
            }

            new_species_collection.push(
                species.clone_with_new_individuals(new_individuals.into_iter())
            );
        }

        GenusSeed::new(
            orphans,
            new_species_collection,
            need_evaluation,
            old_species_individuals_vec)
    }

    /// Generate a new individual from randomly selected parents + mutation
    ///
    /// @param conf Species configuration object
    /// @param population_begin start of the species population
    /// @param pop_end end of the species population
    /// @param selection function to select 1 parent (can be called even if crossover is enabled, when there is not more
    /// than one parent possible)
    /// @param parent_selection function to select 2 parents (only possibly called if crossover is enabled)
    /// @param reproduce_1 function to crossover and create new individuals from 1 parent
    /// @param reproduce_2 function to crossover and create new individuals from 2 parents
    /// @param mutate function that mutates an individual
    /// @return the genus of the next generation
    fn generate_new_individual<'a, 'individual, It, SelectionF, ParentSelectionF, ReproduceI1F, CrossoverI2F, MutateF>(
        &self,
        conf: &Conf,
        mut population: It,
        selection: &'static SelectionF,
        parent_selection: &'static ParentSelectionF,
        reproduce_individual_1: &'static ReproduceI1F,
        crossover_individual_2: &'static CrossoverI2F,
        mutate_individual: &'static MutateF,
    ) -> I
    where
        I: 'individual,
        It: ExactSizeIterator<Item=&'a I> + Sized,
        SelectionF: FnMut(Box<It>) -> &'individual I,
        ParentSelectionF: FnMut(Box<It>) -> (&'individual I,&'individual I),
        ReproduceI1F: FnMut(&I) -> I,
        CrossoverI2F: FnMut(&I, &I) -> I,
        MutateF: FnMut(&mut I),
    {
        let parent_pool_size: usize = population.len();
        assert!(parent_pool_size > 0);

        // Crossover
        let mut child: I =
            if conf.crossover && parent_pool_size > 1 {
                let parents = parent_selection(Box::new(population));
                let parent1 = parents.0;
                let parent2 = parents.1;
                crossover_individual_2(parent1, parent2)
            } else {
                let parent = selection(Box::new(population));
                reproduce_individual_1(parent)
            };

        mutate_individual(&mut child);
        child
    }

    /// Calculates the number of offsprings allocated for each individual.
    /// The total of allocated individuals will be `number_of_individuals`
    ///
    /// @param number_of_individuals Total number of individuals to generate
    /// @return a vector of integers representing the number of allocated individuals for each species.
    /// The index of this list corresponds to the same index in `this->_species_list`.
    fn count_offsprings(&self, number_of_individuals: usize) -> Result<Vec<usize>, String>
    {
        assert!(number_of_individuals > 0);

        let average_adjusted_fitness: F = self.calculate_average_fitness();

        let mut species_offspring_amount: Vec<usize> = self.calculate_population_size(average_adjusted_fitness);

        let mut offspring_amount_sum: usize = species_offspring_amount.iter().sum();
        let missing_offsprings: usize = number_of_individuals - offspring_amount_sum;

        if missing_offsprings != 0 {
            self.correct_population_size(&mut species_offspring_amount, missing_offsprings);
            offspring_amount_sum = species_offspring_amount.iter().sum();

            if offspring_amount_sum != number_of_individuals {
                let error = format!("Generated species_offspring_amount (sum = {}) \
                does not equal number_of_individuals ({}).", offspring_amount_sum, number_of_individuals);
                eprintln!("{}", error);
                return Err(error);
            }
        }

        Ok(species_offspring_amount)
    }
}