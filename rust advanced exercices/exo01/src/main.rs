struct Person<'a> {
    name: &'a str,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_make_person() {
        let person = Person::make_person("name");
        assert_eq!(person.name, "name");
    }
}
