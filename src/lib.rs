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
    use crate::speciation::Individual;

    struct IndividualTest {
        id: usize,
        genome: Vec<bool>,
        fitness: Option<f32>,
    }

    impl IndividualTest {
        pub fn new(id: usize, size: usize) -> Self {
            Self {
                id,
                genome: vec![false; size],
                fitness: None,
            }
        }
    }

    impl Individual<f32> for IndividualTest {
        fn fitness(&self) -> Option<f32> {
            self.fitness
        }

        fn is_compatible(&self, other: &Self) -> bool {
            assert_eq!(self.genome.len(), other.genome.len());
            todo!()
        }
    }

    #[test]
    fn evolution_test() {}
}
