use iter_tools::Itertools;

const LENGTH: usize = 8;

#[derive(PartialEq, Debug, Clone, Eq, Hash)]
enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Brown,
    Orange,
    Black,
    White,
}
fn get_feedback(guess: Vec<Colors>, actual: Vec<Colors>) -> (usize, usize) {
    let mut correct_spot = 0;
    let mut correct_color = 0;
    for (peg_index, peg_color) in guess.iter().enumerate() {
        if actual[peg_index] == *peg_color {
            correct_spot += 1;
        } else if actual.contains(peg_color) {
            correct_color += 1;
        }
    }

    (correct_spot, correct_color)
}

fn main() {
    let colors = vec![
        Colors::Red,
        Colors::Green,
        Colors::Blue,
        Colors::Yellow,
        Colors::Brown,
        Colors::Orange,
        Colors::Black,
        Colors::White,
    ];
    let pin_permutations = {
        let mut permutations = Vec::new();
        let mut indices = vec![0; LENGTH];

        loop {
            if indices.iter().all_unique() {
                permutations.push(
                    indices
                        .iter()
                        .map(|&index| colors[index].clone())
                        .collect_vec(),
                );
            }

            let mut carry = 1;
            for i in (0..LENGTH).rev() {
                indices[i] += carry;
                if indices[i] > colors.len() - 1 {
                    carry = 1;
                    indices[i] = 0;
                } else {
                    carry = 0;
                }
            }

            if carry != 0 {
                break;
            }
        }

        permutations
    };

    let mut possible_results = Vec::new();
    for i in 0..=LENGTH {
        for j in 0..=LENGTH {
            if i + j <= LENGTH {
                possible_results.push((i, j));
            }
        }
    }

    let mut line = rustyline::DefaultEditor::new().unwrap();
    let mut possible_actual_pin_permutations = pin_permutations.clone();
    println!("Possible codes: {}", possible_actual_pin_permutations.len());

    let mut attempt = 0;
    loop {
        let mut best_guess = 0;
        let mut best_score = -1000.0;

        if attempt != 0 {
            for (i, guess_pin_permutation) in pin_permutations.iter().enumerate() {
                let mut result_counts = vec![0; possible_results.len()];

                for actual_pin_permutation in possible_actual_pin_permutations.iter() {
                    let result = get_feedback(
                        guess_pin_permutation.clone(),
                        actual_pin_permutation.clone(),
                    );

                    let result_index = possible_results
                        .iter()
                        .enumerate()
                        .find(|(_, possible_result)| **possible_result == result)
                        .unwrap()
                        .0;
                    result_counts[result_index] += 1;
                }

                let expected_value = result_counts
                    .iter()
                    .map(|count| *count as f32 / pin_permutations.len() as f32 + 0.0000000001)
                    .map(|prob| prob * -prob.log2())
                    .sum::<f32>();
                if expected_value > best_score {
                    best_score = expected_value;
                    best_guess = i;
                }
            }
        }

        println!(
            "\n\n\n\nGuess: {:?}\nFeedback:\n",
            pin_permutations[best_guess]
        );
        let correct_spot_line: usize = line.readline("Correct spot: ").unwrap().parse().unwrap();
        let correct_color_line: usize = line.readline("Correct color: ").unwrap().parse().unwrap();

        possible_actual_pin_permutations = possible_actual_pin_permutations
            .into_iter()
            .filter(|actual| {
                get_feedback(pin_permutations[best_guess].clone(), actual.to_vec())
                    == (correct_spot_line, correct_color_line)
            })
            .collect_vec();

        if possible_actual_pin_permutations.len() == 1 {
            println!("\nThe code is {:?}", possible_actual_pin_permutations[0]);
            return;
        } else {
            println!("Possible codes: {}", possible_actual_pin_permutations.len());
            attempt += 1;
        }
    }
}
