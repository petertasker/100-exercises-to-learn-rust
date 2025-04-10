// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::convert::TryFrom;

impl TryFrom<&str> for Status {
    type Error = ();
    fn try_from(from: &str) -> Result<Status, Self::Error> {
        match from.to_ascii_lowercase().as_str() {
            "done" => Ok(Status::Done),
            "inprogress" => Ok(Status::InProgress),
            "todo" => Ok(Status::ToDo),
            _ => Err(())
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ();
    fn try_from(from: String) -> Result<Status, Self::Error> {
        match from.to_ascii_lowercase().as_str() {
            "done" => Ok(Status::Done),
            "inprogress" => Ok(Status::InProgress),
            "todo" => Ok(Status::ToDo),
            _ => Err(())
        }
    }
}



#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
