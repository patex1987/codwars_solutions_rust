fn create_phone_number(numbers: &[u8]) -> String {
    let value = format!(
        "({}) {}-{}", 
        numbers[0..3].iter().map(|num| num.to_string()).collect::<String>(),
        numbers[3..6].iter().map(|num| num.to_string()).collect::<String>(),
        numbers[6..10].iter().map(|num| num.to_string()).collect::<String>(),
      );
      println!("{}", value);
      value
  }