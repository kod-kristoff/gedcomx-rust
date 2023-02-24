use std::error::Error;
use std::io::Read;
use std::{fs, io};


use gedcomx_model::gedcomx::{verify_gedcomx};
use gedcomx_model::ser::{serialize_to_xml};

use gedcomx_model::GedcomX;

use deserx::DeserializeXml;


use crate::common::emma_bocock_example;

#[test]
fn serialize_as_xml() -> Result<(), Box<dyn Error>> {
    let gedcomx = emma_bocock_example();

    let mut buffer = io::Cursor::new(Vec::new());
    let mut writer = quick_xml::Writer::new_with_indent(&mut buffer, b' ', 4);
    // let mut ser = quick_xml::se::Serializer::with_root(writer, None);
    // gedcomx.serialize(&mut ser)?;

    serialize_to_xml(&gedcomx, &mut writer)?;

    // let buffer = ser.into_inner().into_inner();
    let result = String::from_utf8(buffer.into_inner())?;
    println!("{:#?}", result);

    let mut expected = String::new();
    let mut fp = fs::File::open("assets/data/emma-bocock.xml")?;
    fp.read_to_string(&mut expected)?;
    for (result_line, expected_line) in result.lines().zip(expected.lines()) {
        println!("line='{}'", result_line);
        assert_eq!(result_line, expected_line);
    }
    // assert!(false);
    Ok(())
}

#[test]
fn serialize_as_json() -> Result<(), Box<dyn Error>> {
    let gedcomx = emma_bocock_example();

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

#[test]
fn deserialize_from_json() -> Result<(), Box<dyn Error>> {
    let file = fs::File::open("assets/data/emma-bocock.json")?;
    let reader = io::BufReader::new(file);
    let emma_bocock: GedcomX = serde_json::from_reader(reader)?;
    let verified = verify_gedcomx(&emma_bocock, &emma_bocock_example());
    println!("{:#?}", verified);
    assert!(verified.is_ok());
    Ok(())
}
#[test]
fn deserialize_from_xml() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace"))
        .format_timestamp(None)
        .init();
    let file = fs::File::open("assets/data/emma-bocock.xml")?;
    let reader = io::BufReader::new(file);
    let mut reader = quick_xml::Reader::from_reader(reader);
    let emma_bocock: Result<GedcomX, _> = GedcomX::deserialize_xml(&mut reader);
    println!("{:#?}", emma_bocock);
    if let Err(err) = &emma_bocock {
        println!("source: {:?}", err.source());
    }
    assert!(emma_bocock.is_ok());
    // assert_eq!(emma_bocock?, emma_bocock_example());
    let verified = verify_gedcomx(&emma_bocock?, &emma_bocock_example());
    println!("{:#?}", verified);
    assert!(verified.is_ok());
    Ok(())
}
