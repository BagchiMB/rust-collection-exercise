use rust_collection_exercises::company::add_and_display_employee_interface;
use rust_collection_exercises::median_and_mode::calculate_median_and_mode;
use rust_collection_exercises::median_and_mode::MedianAndMode;
use rust_collection_exercises::pig_latin::calculate_pig_latin_equivalent;

fn main() {
    // Calculate Median and mode of a vector
    let vec1 = vec![1, 4, 3, 2, 2, 5];
    let MedianAndMode { median, mode } = calculate_median_and_mode(vec1);
    println!("Median -> {median}, Mode -> {mode}");

    // Convert words in Pig latin
    let word = String::from("apple");
    let pig_latin_word = calculate_pig_latin_equivalent(word);
    println!("{pig_latin_word}");

    // Text Interface for a company
    add_and_display_employee_interface()
}
