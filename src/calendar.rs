pub mod day_01 {
    pub fn tests() {
        // Test: #1
        let example_1: &str = "1000\n\
                            2000\n\
                            3000\n\
                            \n\
                            4000\n\
                            \n\
                            5000\n\
                            6000\n\
                            \n\
                            7000\n\
                            8000\n\
                            9000\n\
                            \n\
                            10000";
        let result_1: u32 = self::execution(example_1);
        println!("The result is: {}", result_1);

        // Test: #2
        let example_2: &str = "1000\n\
                            2000\n\
                            3000\n\
                            \n\
                            4000\n\
                            30000\n\
                            \n\
                            5000\n\
                            6000\n\
                            \n\
                            7000\n\
                            8000\n\
                            9000\n\
                            \n\
                            10000";
        let result_2: u32 = self::execution(example_2);
        println!("The result is: {}", result_2);
    }

    fn execution(input: &str) -> u32 {
        let mut highest_total: u32 = 0;
        let mut current_total: u32 = 0;
        let mut current_number: String = String::new();

        for current_char in input.chars() {
            if current_char == '\n' {
                if current_number.is_empty() {
                    if current_total > highest_total {
                        highest_total = current_total
                    }
                    current_total = 0;
                    continue;
                }

                current_total =
                    current_total + current_number.parse::<u32>().expect("Not a number!");
                current_number.clear();
                continue;
            }
            current_number.push(current_char);
        }
        highest_total
    }
}
