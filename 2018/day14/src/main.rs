extern crate util;

fn main() {

    /*
    --- Part One ---
    To create new recipes, the two Elves combine their current recipes.
    This creates new recipes from the digits of the sum of the current recipes' scores.
    With the current recipes' scores of 3 and 7, their sum is 10, and so two new recipes would be
    created: the first with score 1 and the second with score 0. If the current recipes' scores
    were 2 and 3, the sum, 5, would only create one recipe (with a score of 5) with its single digit.

    The new recipes are added to the end of the scoreboard in the order they are created.
    So, after the first round, the scoreboard is 3, 7, 1, 0.

    After all new recipes are added to the scoreboard, each Elf picks a new current recipe.
    To do this, the Elf steps forward through the scoreboard a number of recipes equal to 1 plus
    the score of their current recipe. So, after the first round, the first Elf moves
    forward 1 + 3 = 4 times, while the second Elf moves forward 1 + 7 = 8 times. If they run out of
    recipes, they loop back around to the beginning. After the first round, both Elves happen to loop
    around until they land on the same recipe that they had in the beginning; in general, they will
    move to different recipes.

    The Elves think their skill will improve after making a few recipes (your puzzle input). However,
    that could take ages; you can speed this up considerably by identifying the scores of the ten
    recipes after that.
    */

    fn digits_from(n: i32) -> (Option<i32>, Option<i32>) {
        if n > 100 {
            panic!("Out of range")
        }
        (
            /* tens */ if n >= 10 { Some((n / 10) % 10) } else { None }, 
            /* ones */ Some(n % 10)
        )
    }

    /*
    --- Part One ---
    If the Elves think their skill will improve after making 9 recipes, the scores of the ten recipes
    after the first nine on the scoreboard would be 5158916779 (highlighted in the last line of the diagram).
    
    - After 5 recipes, the scores of the next ten would be 0124515891.
    - After 18 recipes, the scores of the next ten would be 9251071085.
    - After 2018 recipes, the scores of the next ten would be 5941429882.
    
    What are the scores of the ten recipes immediately after the number of recipes in your puzzle input?
    */

    let mut recipes = vec![3, 7];
    let mut elf_1: i32 = 0;
    let mut elf_2: i32 = 1;

    let max_recipies_to_test = 920831;

    while recipes.len() < (max_recipies_to_test + 10) {
        
        let combined = recipes[elf_1 as usize] + recipes[elf_2 as usize];
        let (tens, ones) = digits_from(combined);
        recipes.extend(tens.iter());
        recipes.extend(ones.iter());

        elf_1 = (elf_1 + (1 + recipes[elf_1 as usize])) % recipes.len() as i32;
        elf_2 = (elf_2 + (1 + recipes[elf_2 as usize])) % recipes.len() as i32;
    }
        
    let next_ten_recipes = &recipes[max_recipies_to_test..(max_recipies_to_test + 10)];

    println!("Part 1 result: {:?}", next_ten_recipes);

    /*
    --- Part Two ---
    As it turns out, you got the Elves' plan backwards. They actually want to know how many
    recipes appear on the scoreboard to the left of the first recipes whose scores are the
    digits from your puzzle input.

    - 51589 first appears after 9 recipes.
    - 01245 first appears after 5 recipes.
    - 92510 first appears after 18 recipes.
    - 59414 first appears after 2018 recipes.
    
    How many recipes appear on the scoreboard to the left of the score sequence in your puzzle input?
    */

    let mut recipes = vec![3, 7];
    let mut elf_1: i32 = 0;
    let mut elf_2: i32 = 1;

    let pattern_to_search_for = &[9,2,0,8,3,1];

    loop {
        let combined = recipes[elf_1 as usize] + recipes[elf_2 as usize];
        let (tens, ones) = digits_from(combined);
        recipes.extend(tens.iter());
        recipes.extend(ones.iter());

        elf_1 = (elf_1 + (1 + recipes[elf_1 as usize])) % recipes.len() as i32;
        elf_2 = (elf_2 + (1 + recipes[elf_2 as usize])) % recipes.len() as i32;

        // Check the end of the vector for our pattern
        let result = recipes
            .windows(pattern_to_search_for.len()).enumerate().rev().take(2)
            .filter(|(_, w)| w == pattern_to_search_for)
            .next();

        if let Some((i, _)) = result {
            println!("Part 2 result: {:?}", i);
            break;
        }
    }
}