use crate::common::emma_bocock_example;

mod persons {
    use gedcomx_model::types::FactType;

    use super::*;
    #[test]
    fn test_emma() {
        let gedcom = emma_bocock_example();
        let person = &gedcom.persons()[0];
        assert_eq!(
            person.names()[0].name_forms()[0].get_full_text(),
            "Emma Bocock"
        );
        assert!(person.is_extracted());
        assert_eq!(person.facts()[0].r#type(), FactType::Birth);
    }
}
