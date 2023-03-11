#[derive(Debug)]
pub struct HighScores {
    pub high_scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            high_scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.high_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.high_scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.high_scores.iter().max().copied()
    }

    fn ascending_cmp(a: &u32, b: &u32) -> std::cmp::Ordering {
        b.cmp(a)
    }

    pub fn personal_top_three<'a>(&self) -> Vec<u32> {
        let mut top_three = vec![];

        for score in &self.high_scores {
            if top_three.len() < 3 {
                top_three.push(*score);
                top_three.sort_by(Self::ascending_cmp);
            } else if top_three[2] < *score {
                top_three[2] = *score;
                top_three.sort_by(Self::ascending_cmp);
            }
        }

        top_three
    }
}
