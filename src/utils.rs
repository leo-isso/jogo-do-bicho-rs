pub fn normalize_bet_numbers(number: usize) -> String {
    String::from(format!("{:0zbefore$}", number, zbefore=4))       
}
