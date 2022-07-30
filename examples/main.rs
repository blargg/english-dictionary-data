use english_dictionary_data::all_words;

fn main() {
    for word in all_words() {
        println!("{word}");
    }
}