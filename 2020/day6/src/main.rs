extern crate util;

use std::collections::{ HashMap };

use util::{ read_input };
use util::error::{ AppErr };

fn main() -> Result<(), AppErr> {
    /*
    --- Part One ---
    
    For each of the people in their group, you write down the questions for which they answer "yes", one per line.
    For example:

        abcx
        abcy
        abcz

    In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z.
    (Duplicate answers to the same question don't count extra; each question counts at most once.)


        
    For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
    
    */

    #[derive(Debug)]
    struct AnswerGroup {
        answers: Vec<Vec<char>>,
    }

    fn try_read_answer_group(input: &mut dyn Iterator<Item=&str>) -> Result<Option<AnswerGroup>, AppErr> {

        let answers = input
            .take_while(|line| line.trim().len() > 0)
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if answers.len() == 0 {
            return Ok(None);
        }
        let answer_group = AnswerGroup {
            answers
        };
        Ok(Some(answer_group))
    }

    let input = read_input("input.txt")?;
    let mut input_reader = input.iter().map(|s| s.as_str());
    let mut answer_groups = Vec::new();
    while let Some(group) = try_read_answer_group(&mut input_reader)? {
        answer_groups.push(group);
    }

    let group_answer_counts_sum =
        answer_groups.iter()
            .map(|g| {
                let mut hash = HashMap::new();
                for answers in &g.answers {
                    for key in answers {
                        *hash.entry(key).or_insert(0) += 1;
                    }
                }
                hash.keys().len() as u32
            })
            .sum::<u32>();

    println!("Part 1: sum of questions answered for all groups: {}", group_answer_counts_sum);

    Ok(())
}
