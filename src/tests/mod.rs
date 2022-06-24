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

use std::ptr;

use rand::prelude::*;

use crate::speciation::{Conf, Genus, Individual};

#[derive(Clone)]
struct IndividualTest {
    id: usize,
    genome: Vec<bool>,
    fitness: Option<f32>,
}

impl IndividualTest {
    pub fn empty(id: usize, size: usize) -> Self {
        Self {
            id,
            genome: vec![false; size],
            fitness: None,
        }
    }
    pub fn random(id: usize, size: usize, rng: &mut ThreadRng) -> Self {
        Self {
            id,
            genome: (0..size).into_iter().map(|_| rng.gen()).collect(),
            fitness: None,
        }
    }

    pub fn evaluate(&mut self) -> f32 {
        let fitness = self.genome.iter().map(|i| if *i { 1.0 } else { 0.0 }).sum();
        self.fitness = Some(fitness);
        fitness
    }

    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        use rand::distributions::Uniform;
        let pos = Uniform::from(0..self.genome.len()).sample(rng);
        self.genome[pos] = !self.genome[pos];
    }

    pub fn crossover(&self, other: &Self, new_id: usize, rng: &mut ThreadRng) -> Self {
        let mut new_indiv = Self::empty(new_id, 0);

        if ptr::eq(self, other) {
            new_indiv.genome = self.genome.clone();
        } else {
            use rand::distributions::Uniform;
            let swap_point = Uniform::from(0..self.genome.len()).sample(rng);
            new_indiv.genome = self.genome.iter()
                .take(swap_point)
                .chain(other.genome.iter().skip(swap_point))
                .cloned()
                .collect();

            assert_eq!(self.genome.len(), new_indiv.genome.len());
        }

        new_indiv
    }
}

impl Individual<f32> for IndividualTest {
    fn fitness(&self) -> Option<f32> {
        self.fitness
    }

    fn is_compatible(&self, other: &Self) -> bool {
        assert_eq!(self.genome.len(), other.genome.len());
        let distance: usize =
            self.genome.iter().zip(other.genome.iter())
                .map(|(s, o)| if s == o { 0 } else { 1 })
                .sum();
        distance > (self.genome.len() / 3)
    }
}

#[test]
fn evolution_test() {
    const POPULATION_SIZE: usize = 10;
    const GENOME_SIZE: usize = 10;
    const MAX_GENERATIONS: usize = 100;
    let mut rng = rand::thread_rng();

    let mut genus: Genus<IndividualTest, f32> = crate::speciation::Genus::new();
    let initial_population: Vec<IndividualTest> = (0..POPULATION_SIZE).into_iter()
        .map(|i| IndividualTest::random(i, GENOME_SIZE, &mut rng))
        .collect();

    let mut id_counter = initial_population.len();

    genus.speciate(initial_population.into_iter());
    assert_eq!(genus.count_individuals(), POPULATION_SIZE);

    let conf = Conf {
        total_population_size: POPULATION_SIZE,
        crossover: true,
        young_age_threshold: 2,
        old_age_threshold: 10,
        species_max_stagnation: 20,
        young_age_fitness_boost: 1.1,
        old_age_fitness_penalty: 0.9,
    };

    let mut best_fitness = f32::NEG_INFINITY;


    // LAMBDA FUNCTIONS FOR GENOTYPE OPERATIONS
    // let selection = |mut it| it.next().unwrap();
    //
    // let parent_selection = |mut it | { (it.next(), it.next()) };

    let mut crossover_1 = |parent: &IndividualTest| {
        let mut child = parent.clone();
        child.id = id_counter;
        id_counter +=1;
        child
    };

    let mut crossover_2 = |parent1: &IndividualTest, parent2: &IndividualTest| {
        let child = parent1.crossover(parent2, id_counter, &mut rng);
        id_counter +=1;
        child
    };

    let mut mutate = |individual: &mut IndividualTest| {
        individual.mutate(&mut rng)
    };

    let population_manager = || {};

    let evaluate = |new_individual: &mut IndividualTest| {
        let fitness = new_individual.evaluate();
        if fitness > best_fitness {
            best_fitness = fitness;
        }
        fitness
    };

    // EVOLUTION START

    let mut generation_n: usize = 0;

    genus.ensure_evaluated_population(evaluate);

    while best_fitness < GENOME_SIZE as f32 {
        generation_n += 1;
        println!("Starting generation {}", generation_n);
        let mut generated_individuals = genus.update(&conf)
            .generate_new_individuals(
                &conf,
                &mut |mut it| it.next().unwrap(),
                &mut |mut it| (it.next().unwrap(), it.next().unwrap()),
                &mut crossover_1,
                &mut crossover_2,
                &mut mutate,
            );

        generated_individuals.evaluate(evaluate);

        genus = genus.next_generation(&conf,
                                      generated_individuals,
                                      population_manager);

        if generation_n > MAX_GENERATIONS {
            assert!(false);
        }
    }

    println!("Evolution took {} generations to complete with a fitness of {}", generation_n, best_fitness);
}
