use std::error::Error;
use std::io::Read;
use std::{fs, io};

use gedcomx_model::agent::Agent;
use gedcomx_model::common::DateTime;
use gedcomx_model::conclusion::{Date, Document, Fact, Person, PlaceReference, Relationship};
use gedcomx_model::gedcomx::Attribution;
use gedcomx_model::ser::{serialize_to_xml, XmlSerializer};
use gedcomx_model::source::{SourceCitation, SourceDescription};
use gedcomx_model::types::{FactType, Gender, RelationshipType, ResourceType};
use gedcomx_model::GedcomX;

fn emma_bocock() -> GedcomX {
    let contributor = Agent::new()
        .id("A-1")
        .name("Jane Doe")
        .email("example@example.org");
    let repository = Agent::new()
        .id("A-2")
        .name("General Registry Office, Southport");
    let attribution = Attribution::new().contributor(&contributor).modified(
        "2014-03-07T00:00:00-07:00"
            .parse::<DateTime>()
            .expect("failed parsing date"),
    );
    let source_description = SourceDescription::new()
        .id("S-1")
    .title("Birth Certificate of Emma Bocock, 23 July 1843, General Registry Office")
      .citation(SourceCitation::new().value("England, birth certificate for Emma Bocock, born 23 July 1843; citing 1843 Birth in District and Sub-district of Ecclesall-Bierlow in the County of York, 303; General Registry Office, Southport."))
      .resource_type(ResourceType::PhysicalArtifact)
      .created("1843-07-27T00:00:00-07:00".parse::<DateTime>().expect("failed"))
      .repository(&repository);
    let birth = Fact::new(FactType::Birth)
        .date(Date::new().original("23 June 1843"))
        .place(PlaceReference::new().original(
            "Broadfield Bar, Abbeydale Road, Ecclesall-Bierlow, York, England, United Kingdom",
        ));
    let emma = Person::with_id("P-1")
        .extracted(true)
        .source(&source_description)
        .name("Emma Bocock")
        .gender(Gender::Female)
        .fact(birth);
    let father = Person::with_id("P-2")
        .extracted(true)
        .source(&source_description)
        .name("William Bocock")
        .fact(Fact::new(FactType::Occupation).value("Toll Collector"));
    let mother = Person::with_id("P-3")
        .extracted(true)
        .source(&source_description)
        .name("Sarah Bocock formerly Brough");
    let father_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&father)
        .person2(&emma);

    let mother_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&mother)
        .person2(&emma);

    let analysis = Document::new()
        .id("D-1")
        .text("...Jane Doe's analysis document...");
    let emma_conclusion = Person::with_id("C-1").evidence(&emma).analysis(&analysis);
    GedcomX::new()
        .agent(contributor)
        .agent(repository)
        .attribution(attribution)
        .source_description(source_description)
        .person(emma)
        .person(father)
        .person(mother)
        .relationship(father_relationship)
        .relationship(mother_relationship)
        .document(analysis)
        .person(emma_conclusion)
}

#[test]
fn serialize_as_xml() -> Result<(), Box<dyn Error>> {
    let gedcomx = emma_bocock();

    let mut writer = XmlSerializer::new_with_indent(io::Cursor::new(Vec::new()), b' ', 4);

    serialize_to_xml(&gedcomx, &mut writer)?;

    let result = writer.into_inner().into_inner();
    let result = String::from_utf8(result)?;

    let mut expected = String::new();
    let mut fp = fs::File::open("assets/data/emma-bocock.xml")?;
    fp.read_to_string(&mut expected)?;
    for (result_line, expected_line) in result.lines().zip(expected.lines()) {
        println!("line='{}'", result_line);
        assert_eq!(result_line, expected_line);
    }
    Ok(())
}

#[test]
fn serialize_as_json() -> Result<(), Box<dyn Error>> {
    let gedcomx = emma_bocock();

    let result = serde_json::to_string_pretty(&gedcomx)?;

    let mut expected = String::new();
    let mut fp = fs::File::open("assets/data/emma-bocock.json")?;
    fp.read_to_string(&mut expected)?;
    for (result_line, expected_line) in result.lines().zip(expected.lines()) {
        println!("line='{}'", result_line);
        assert_eq!(result_line, expected_line);
    }
    Ok(())
}
