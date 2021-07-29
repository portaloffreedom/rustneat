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
pub mod speciation;

#[cfg(test)]
mod tests {
    use std::ptr;

    use rand::prelude::*;

    use crate::speciation::{Genus, Individual};

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

        pub fn evaluate(&mut self) {
            self.fitness = Some(self.genome.iter().map(|i| if *i { 1.0 } else { 0.0 }).sum())
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
        let mut rng = rand::thread_rng();

        let genus: Genus<IndividualTest, f32> = crate::speciation::Genus::new();
        let initial_population: Vec<IndividualTest> = (0..POPULATION_SIZE).into_iter()
            .map(|i| IndividualTest::random(i, GENOME_SIZE, &mut rng))
            .collect();
    }
}
