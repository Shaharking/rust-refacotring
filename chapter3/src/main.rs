
struct Matcher {
}

impl Matcher {
    fn Match(&self, expected: &mut [i32], actual: &mut [i32], clip_limit: i32, delta: u32) -> bool {
        // Clip "too-large" values
        for x in actual.iter_mut() {
            if *x > clip_limit {
                *x = clip_limit;
            }
        } 
        // Check for length differences
        if actual.len() != expected.len() {
            return false;
        }
        // Check that each entry within expected +/- delta
        for x in 1..actual.len() {
            if actual[x].abs_diff(expected[x]) > delta {
                return false;
            }
        }

        return true;
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn it_should_match() {
        let matcher = Matcher {};
        let mut expected = [10, 50, 30, 98];
        let mut actual = [12, 55, 25, 110];
        let result = matcher.Match(&mut expected, &mut actual, 100, 5);
        
        assert_eq!(result, true);
    }

    #[test]
    fn it_should_not_match() {
        let matcher = Matcher {};
        let mut expected = [10, 50, 30, 98];
        let mut actual = [10, 60, 30, 98];
        let result = matcher.Match(&mut expected, &mut actual, 100, 5);
        
        assert_eq!(result, false);
    }
}