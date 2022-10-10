
pub fn is_opt(gender: String) -> Option<bool>{
        if gender == String::from('M') || gender == String::from('F'){
            return Some(true);
        }
        else {
            return None;
        }
    }

