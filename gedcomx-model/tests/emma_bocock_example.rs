use std::error::Error;
use std::io::Read;
use std::{fs, io};

use gedcomx_model::agent::Agent;
use gedcomx_model::common::Attribution;
use gedcomx_model::conclusion::Person;
use gedcomx_model::ser::{serialize_to_xml, XmlSerializer};
use gedcomx_model::GedcomX;

fn emma_bocock() -> GedcomX {
    let contributor = Agent::new()
        .id("A-1".into())
        .name("Jane Doe".into())
        .email("example@example.org");
    let attribution = Attribution::new().contributor(contributor.clone());
    GedcomX::new()
        .agent(contributor)
        .attribution(attribution)
        .person(Person::new().id("P-1".into()).extracted(true))
}

const EXPECTED_XML: &str = r#"<gedcomx>
  <person extracted="true" id="P-1">
  </person>
</gedcomx>"#;

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
    for (result_line, expected_line) in expected.lines().zip(result.lines()) {
        assert_eq!(result_line, expected_line);
    }
    Ok(())
}
