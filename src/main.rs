use serde::Deserialize;
use serde_xml_rs::from_str;
use anyhow::Result;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ram:Something")]
struct Something
{
    #[serde(rename = "ram:InSomethingElse")]
    in_something_else: String,
}

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
    #[serde(rename = "ram:Something")]
    something: Something,
}

fn main() -> Result<()> {

    let xml = r#"
<Invoice xmlns:ram="urn:un:unece:uncefact:data:standard:ReusableAggregateBusinessInformationEntity:100">
    <ram:ID>2025-12522</ram:ID>
    <ram:TypeCode>380</ram:TypeCode>
    <ram:Content>accadis International School gemeinnützige GmbH
Am Weidenring 52 - 54
61352 Bad Homburg
Deutschland</ram:Content>
    <ram:SubjectCode>REG</ram:SubjectCode>
    <ram:Something>
        <ram:InSomethingElse>just a test for understanding</ram:InSomethingElse>
    </ram:Something>
</Invoice>
    "#;

    let invoice: Invoice = from_str(xml)?;
    println!("Invoice: {:#?}", invoice);

    Ok(())
}