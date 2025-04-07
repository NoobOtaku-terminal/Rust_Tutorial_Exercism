use std::collections::{HashMap, HashSet};

pub struct School {
    grades: HashMap<u32, HashSet<String>>,
    students: HashSet<String>,  // Track all students across all grades
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
            students: HashSet::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        // Only add the student if they're not already in any grade
        if self.students.insert(student.to_string()) {
            self.grades
                .entry(grade)
                .or_insert_with(HashSet::new)
                .insert(student.to_string());
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grade_levels: Vec<u32> = self.grades.keys().cloned().collect();
        grade_levels.sort();
        grade_levels
    }

    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.grades.get(&grade) {
            None => Vec::new(),
            Some(students) => {
                let mut student_list: Vec<String> = students.iter().cloned().collect();
                student_list.sort();
                student_list
            }
        }
    }
}