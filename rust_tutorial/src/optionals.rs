// all file is a module by default all functions are private.

pub fn is_opt(gender: String) -> Option<bool>{
        if gender == String::from('M') || gender == String::from('F'){
            return Some(true);
        }
        else {
            return None;
        }
    }

pub fn is_err(gender: String) -> Result<bool, String>{
        if gender == String::from('M') || gender == String::from('F'){
            return Ok(true);
        }
        else {
            return Err(format!("illegal gender data provided:{}",gender));
        }
    }
