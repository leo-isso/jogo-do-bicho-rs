pub fn normalize_bet_numbers(number: u16) -> String {
    String::from(format!("{:0zbefore$}", number, zbefore=4))       
}
