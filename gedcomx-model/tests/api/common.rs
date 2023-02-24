use gedcomx_model::agent::Agent;
use gedcomx_model::common::{DateTime, IriRef};
use gedcomx_model::conclusion::{Date, Document, Fact, Person, PlaceReference, Relationship};
use gedcomx_model::gedcomx::{Attribution};
use gedcomx_model::source::{SourceCitation, SourceDescription};
use gedcomx_model::types::{FactType, Gender, RelationshipType, ResourceType};
use gedcomx_model::GedcomX;

pub fn iri(s: &str) -> IriRef {
    IriRef::parse(s.into()).expect("parse iri")
}
pub fn emma_bocock_example() -> GedcomX {
    let contributor = Agent::new(iri("#A-1"))
        .name("Jane Doe")
        .try_email("example@example.org")
        .expect("email");
    let repository = Agent::new(iri("#A-2")).name("General Registry Office, Southport");
    let attribution = Attribution::new().contributor(&contributor).modified(
        "2014-03-07T00:00:00-07:00"
            .parse::<DateTime>()
            .expect("failed parsing date"),
    );
    let source_description = SourceDescription::new(
        iri("#S-1"))
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
    let emma = Person::new(iri("#P-1"))
        .extracted(true)
        .source(&source_description)
        .name("Emma Bocock")
        .gender(Gender::Female)
        .fact(birth);
    let father = Person::new(iri("#P-2"))
        .extracted(true)
        .source(&source_description)
        .name("William Bocock")
        .fact(Fact::new(FactType::Occupation).value("Toll Collector"));
    let mother = Person::new(iri("#P-3"))
        .extracted(true)
        .source(&source_description)
        .name("Sarah Bocock formerly Brough");
    let father_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&father)
        .person2(&emma);

    let mother_relationship = Relationship::new(RelationshipType::ParentChild)
        .person1(&mother)
        .person2(&emma);

    let analysis = Document::new(iri("#D-1")).text("...Jane Doe's analysis document...");
    let emma_conclusion = Person::new(iri("#C-1")).evidence(&emma).analysis(&analysis);
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
