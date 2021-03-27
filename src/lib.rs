use std::fmt;

// Represents a solution for a Grid with a dictionary
// TODO: we may want to implement Copy (and thus not use Vec)
pub struct Solution {
  words_found: Vec<String>,
  score: u32
}
impl Solution {
  /** Form a solutuion with a list of words found, as well as a total score for the solution.
   */
  pub fn new(words_found: Vec<String>, score: u32) -> Solution {
    Solution { words_found, score }
  }
}
impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let res: fmt::Result = write!(f, "score: {}, words: ", self.score);
      if res.is_err() { return res }

      for word in &self.words_found {
        let res: fmt::Result = write!(f, "{}, ", word);
        if res.is_err() { return res }
      }
      write!(f, "")
    }
}