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
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::Debug;
use std::rc::Rc;

use crate::speciation::{Conf, Individual, Species};
use crate::speciation::genus_seed::GenusSeed;
use crate::speciation::species::{RcSpecies, SpeciesIter};
use crate::util::iterators::has_unique_elements;

use super::species_collection::SpeciesCollection;

pub struct Genus<I: Individual<F>, F: num::Float> {
    next_species_id: usize,
    species_collection: SpeciesCollection<I, F>,
}

impl<I, F> Genus<I, F>
where
    I: 'static + Individual<F> + Debug,
    F: 'static + num::Float + Debug + std::iter::Sum,
{
    /// Creates a new Genus object
    pub fn new() -> Self {
        Self {
            next_species_id: 1,
            species_collection: SpeciesCollection::new(),
        }
    }

    fn build_next_generation(species_collection: SpeciesCollection<I, F>, next_species_id: usize) -> Self {
        Self {
            next_species_id,
            species_collection
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

    pub fn ensure_evaluated_population<E: FnMut(&mut I) -> F>(&mut self, mut evaluate_individual: E)
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
        &'a mut self,
        conf: &Conf,
        selection: &mut SelectionF,
        parent_selection: &mut ParentSelectionF,
        reproduce_individual_1: &mut ReproduceI1F,
        crossover_individual_2: &mut CrossoverI2F,
        mutate_individual: &mut MutateF,
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
        let offspring_amounts: Vec<usize> = self.count_offsprings(conf.total_population_size)
            .expect("count offspring to be successful");

        // Clone Species
        let mut new_species_collection: Vec<RcSpecies<I,F>> = Vec::new();
        let mut orphans: Vec<Rc<RefCell<I>>> = Vec::new();

        // Pointers to values in new_species_collection and orphans
        let mut need_evaluation: Vec<Rc<RefCell<I>>> = Vec::new();

        //////////////////////////////////////////////
        // GENERATE NEW INDIVIDUALS
        for (species_i, species) in self.species_collection.iter().enumerate() {

            let mut new_individuals: Vec<Rc<RefCell<I>>> = Vec::new();

            for n_offspring in 0_usize..offspring_amounts[species_i] {
                for _ in 0..n_offspring {
                    let new_individual: Rc<RefCell<I>> = Rc::new(RefCell::new(
                        Self::generate_new_individual::<
                            SpeciesIter<'a, I, F>,
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
                        )));

                    // if the new individual is compatible with the species, otherwise create new.
                    need_evaluation.push(new_individual.clone());
                    if species.is_compatible(&new_individual.borrow()) {
                        new_individuals.push(new_individual);
                    } else {
                        orphans.push(new_individual);
                    }
                }
            }

            new_species_collection.push(
                species.clone_with_new_individuals(new_individuals.into_iter())
            );
        };

        // Pointers to current const species_collection
        let old_species_individuals_vec = {
            self.species_collection.iter_mut()
                .map(|species| species.drain_individuals().collect()).collect()
        };

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
        conf: &Conf,
        population: It,
        selection: &mut SelectionF,
        parent_selection: &mut ParentSelectionF,
        reproduce_individual_1: &mut ReproduceI1F,
        crossover_individual_2: &mut CrossoverI2F,
        mutate_individual: &mut MutateF,
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
    fn count_offsprings(&mut self, number_of_individuals: usize) -> Result<Vec<usize>, String>
    {
        assert!(number_of_individuals > 0);

        let average_adjusted_fitness: F = self.calculate_average_fitness().expect("Couldn't calculate average fitness");

        let mut species_offspring_amount: Vec<usize> = self.calculate_population_size(average_adjusted_fitness);

        let mut offspring_amount_sum: usize = species_offspring_amount.iter().sum();
        let missing_offsprings = number_of_individuals as i32 -  offspring_amount_sum as i32;

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

    /// Calculates the Average fitness of the population based on the adjusted fitnesses
    ///
    /// @return the average fitness
    fn calculate_average_fitness(&self) -> Result<F,&str> {
        // Calculate the total adjusted fitness
        let mut total_adjusted_fitness: F = F::zero();
        let mut number_of_individuals: usize = 0;
        for species in self.species_collection.iter() {
            total_adjusted_fitness = total_adjusted_fitness + species.accumulated_adjusted_fitness();
            number_of_individuals += species.len();
        }
        if total_adjusted_fitness <= F::zero() {
            return Err("Total adjusted fitness is <= 0");
        }

        // Calculate the average adjusted fitness
        let average_adjusted_fitness: F = total_adjusted_fitness / F::from(number_of_individuals).unwrap();

        Ok(average_adjusted_fitness)
    }

    /// Calculates the number of offsprings allocated for each individual given the `average_adjusted_fitness`.
    /// The function is rounding real numbers to integer numbers, so the returned vector quite possibly will not sum up
    /// to the total population size.
    ///
    /// @param average_adjusted_fitness The average adjusted fitness across all the species.
    /// @return a vector of integers representing the number of allocated individuals for each species.
    /// The index of this list corresponds to the same index in `self.species_list`.
    fn calculate_population_size(&self, average_adjusted_fitness: F) -> Vec<usize>
    {

        let species_offspring_amount: Vec<_> = self.species_collection.iter()
            .map(|species| {
                // each species amount is given by the sum of the fitness
                // of the individuals normalized by the average_adjusted_fitness
                let offspring_amount: F = species.accumulated_adjusted_fitness() / average_adjusted_fitness;
                offspring_amount.floor().to_usize().unwrap()
            }).collect();

        return species_offspring_amount;

    }

    /// `species_offspring_amount` could be incorrect because of approximation errors when we round floats to integers.
    ///
    /// This method modifies the `species_offspring_amount` so that the sum of the vector is equal to the total population size.
    /// It adds (or removes if negative) the `missing_offspring` number of individuals in the vector.
    /// When adding, it chooses the best species.
    /// When removing, it chooses the worst species, multiple species if one species is not big enough.
    ///
    /// @param species_offspring_amount vector of offspring_amounts that needs correction
    /// @param missing_offspring amount of correction to be done. Positive means we need more offsprings, negative means
    /// we have to much.
    fn correct_population_size(&mut self, species_offspring_amount: &mut Vec<usize>, missing_offspring: i32)
    {
        // positive means lacking individuals
        if missing_offspring > 0
        {
            let i: usize = self.species_collection.get_best().expect("a best species to be found");
            species_offspring_amount[i] += missing_offspring as usize;
        }
        // negative have excess individuals
        else if missing_offspring < 0
        {
            // remove missing number of individuals
            let mut excess_offspring = (-missing_offspring) as usize;
            let mut excluded_id_list= HashSet::<usize>::new();

            while excess_offspring > 0 {
                let (worst_species_i, worst_species) = self.species_collection
                    .get_worst(1, Some(&excluded_id_list)).expect("Couldn't find the worst species");

                let mut current_amount = species_offspring_amount[worst_species_i];

                if current_amount > excess_offspring {
                    current_amount -= excess_offspring;
                    excess_offspring = 0;
                } else {
                    excess_offspring -= current_amount;
                    current_amount = 0;
                }

                species_offspring_amount[worst_species_i] = current_amount;
                excluded_id_list.insert(worst_species.id);
            }

            assert_eq!(excess_offspring, 0);
        }
        else
        {
        eprintln!("missing_offspring == 0, why did you call correct_population_size()?");
        }
    }

    pub fn next_generation<PopManager>(&mut self,
                           conf: &Conf,
                           generated_individuals: GenusSeed<I, F>,
                           mut population_management: PopManager) -> Self
    where
        PopManager: FnMut(Vec<I>, Vec<I>, usize) -> Vec<I>
    {
        let mut local_next_species_id: usize = self.next_species_id;

        let mut new_species_collection = SpeciesCollection::new_from_iter(
            generated_individuals.new_species_collection
                .into_iter()
                .map(|rc_species| rc_species.promote())
        );

        //////////////////////////////////////////////
        // MANAGE ORPHANS, POSSIBLY CREATE NEW SPECIES
        // recheck if other species can adopt the orphans individuals.

        for orphan in generated_individuals.orphans {
            let orphan = Rc::try_unwrap(orphan).unwrap().into_inner();
            let compatible_species = new_species_collection.iter_mut()
                .find(|species| species.is_compatible(&orphan));

            if let Some(compatible_species) = compatible_species {
                compatible_species.insert(orphan);
            } else {
                let new_species = Species::new(orphan, local_next_species_id);
                local_next_species_id += 1;
                new_species_collection.push(new_species);
                // add an entry for new species which does not have a previous iteration.
                }
            }

        // Do a recount on the number of offspring per species
        let new_population_size = 0; //TODO list_of_new_species.count_individuals();
        let offspring_amounts = self.count_offsprings(conf.total_population_size - new_population_size).unwrap();
        // If this assert fails, the next population size is going to be different
        assert_eq!(offspring_amounts.iter().sum::<usize>(), conf.total_population_size - new_population_size);


        //////////////////////////////////////////////
        // POPULATION MANAGEMENT
        // update the species population, based ont he population management algorithm.
        for (species_i, (new_species, old_species_individuals))
        in new_species_collection.iter_mut()
            .zip(generated_individuals.old_species_individuals.into_iter())
            .enumerate()
        {
            if species_i > self.species_collection.len() {
                //TODO probably not needed because of .zip()
                // Finished. The new species keep the entire population.
                println!("POPULATION MANAGEMENT Finished. The new species keep the entire population.");
                break;
            }
            println!("POPULATION MANAGEMENT {}", species_i);

            // this empties the new_species list
            println!("POPULATION MANAGEMENT {} transform", species_i);
            let new_species_individuals = new_species.drain_individuals().collect();

            println!("POPULATION MANAGEMENT {} lambda call", species_i);
            // Create next population
            let new_individuals = population_management(
                new_species_individuals,
                old_species_individuals,
                offspring_amounts[species_i]);

            new_species.set_individuals(new_individuals.into_iter());

            println!("POPULATION MANAGEMENT {} done", species_i);
        }


        //////////////////////////////////////////////
        // ASSERT SECTION
        // check for duplicated species IDs
        assert!(has_unique_elements(
            new_species_collection.iter()
                .map(|species| species.id)));

        new_species_collection.cleanup();

        // Assert species list size and number of individuals
        let n_individuals: usize = new_species_collection.count_individuals();
        if n_individuals != conf.total_population_size {
            panic!("count_individuals(new_species_collection) = {} != {} = population_size",
                n_individuals, conf.total_population_size);
        }

        //////////////////////////////////////////////
        // CREATE THE NEXT GENUS
        Genus::build_next_generation(new_species_collection, local_next_species_id)
    }
}