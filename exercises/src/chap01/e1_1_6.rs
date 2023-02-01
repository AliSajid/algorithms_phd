use crate::Exercise;

#[derive(Debug)]
pub struct Solution;

impl Exercise for Solution {
    fn solve(&self, exercise: String) -> String {
        format!("Input: {}", exercise);
        exercise
    }
}
