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

#[derive(Clone)]
pub struct Age {
    pub generations: usize,
    pub evaluations: usize,
    pub no_improvements: usize,
}

impl Age {
    pub fn new() -> Self {
        Self {
            generations: 0,
            evaluations: 0,
            no_improvements: 0,
        }
    }

    // Increasers
    pub fn increase_generations(&mut self) { self.generations+=1; }
    pub fn increase_evaluations(&mut self) { self.evaluations+=1; }
    pub fn increase_no_improvements(&mut self) { self.no_improvements+=1; }

    // Resetters

    /// Makes the age young again
    pub fn reset_generations(&mut self) {
        self.generations = 0;
        self.no_improvements = 0;
    }

    pub fn reset_no_improvements(&mut self) {
        self.no_improvements = 0;
    }

    pub fn reset_evaluations(&mut self) {
        self.evaluations = 0;
    }
}