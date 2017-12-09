fn main() {
    let words = [
        "ptolemaic",
        "retrograde",
        "supplant",
        "undulate",
        "xenoepist",
        "asymptote",
        "babka",
        "banoffee",
        "engender",
        "karpatka",
        "othellolagkage",
    ];

    let mut low_index = 0;
    let mut high_index = words.len() - 1;

    loop {
        let test_index = low_index + ((high_index - low_index) / 2);

        let low_char = words[low_index].bytes().next().unwrap();
        let test_char = words[test_index].bytes().next().unwrap();

        println!("low:{}\ttest:{}\thigh:{}", low_index, test_index, high_index);

        if low_char > test_char {
            high_index = test_index;
        } else {
            low_index = test_index;
        }

        if low_index == high_index -1 {
            println!("{}", test_index);
            break;
        }
    }


    /* for (i, word) in words.iter().enumerate() {
        let test_letter = word.bytes().next().unwrap();
        if test_letter < selected_char  {
            selected_char = test_letter;
            selected_index = i;
            break;
        }
    } */
}
