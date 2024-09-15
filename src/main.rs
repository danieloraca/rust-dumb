fn get_values() -> (usize, i32) {
    let yo: usize = (-1 as i32) as usize;
    let ho: i32 = -1 as i32;

    (yo, ho)
}

fn main() {
    let (yo, ho) = get_values();

    println!("{}", yo);
    println!("{}", ho);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yo_value() {
        let (yo, _) = get_values();

        #[cfg(target_pointer_width = "64")]
        assert_eq!(yo, 18446744073709551615);
    }

    #[test]
    fn test_ho_value() {
        let (_, ho) = get_values();

        assert_eq!(ho, -1);
    }
}
