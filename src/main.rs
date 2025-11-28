use serde::Deserialize;
use serde_xml_rs::from_str;
use std::error::Error;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "Invoice")]
struct Invoice
{
    #[serde(rename = "ram:ID")]
    id: String,
    #[serde(rename = "ram:TypeCode")]
    type_code: String,
    #[serde(rename = "ram:Content")]
    content: String,
    #[serde(rename = "ram:SubjectCode")]  
    subject_code: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    let xml = r#"
<Invoice xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100">
    <ram:ID>2025-12522</ram:ID>
    <ram:TypeCode>380</ram:TypeCode>
    <ram:Content>accadis International School gemeinnützige GmbH
Am Weidenring 52 - 54
61352 Bad Homburg
Deutschland</ram:Content>
    <ram:SubjectCode>REG</ram:SubjectCode>
</Invoice>
    "#;

    let invoice: Invoice = from_str(xml).expect("Failed to parse XML");
    println!("Invoice: {:#?}", invoice);

    Ok(())
}