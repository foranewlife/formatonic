use std::collections::HashMap;

/// HashMap to store all fields
#[derive(Debug, Default)]
pub struct UPFData {
    pub version: String,
    pub fields: HashMap<String, Block>,
}

/// Data for a single field
#[derive(Debug)]
pub struct Block {
    pub attributes: HashMap<String, String>,
    pub raw_content: String,
    pub sub_blocks: HashMap<String, Vec<Block>>, // Subfields (optional)
}

use xml::reader::{EventReader, XmlEvent};

impl UPFData {
    /// Parse from string
    pub fn parse(input: &str) -> Result<Self, xml::reader::Error> {
        let mut data = UPFData::default();
        let parser = EventReader::from_str(input);
        let mut stack: Vec<Block> = Vec::new();

        for event in parser {
            match event? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let mut attrs = HashMap::new();
                    for attr in attributes {
                        attrs.insert(attr.name.local_name, attr.value);
                    }

                    let block = Block {
                        attributes: attrs,
                        raw_content: String::new(),
                        sub_blocks: HashMap::new(),
                    };

                    if name.local_name == "UPF" {
                        if let Some(version) = block.attributes.get("version") {
                            data.version = version.clone();
                        }
                    } else {
                        stack.push(block);
                    }
                }
                XmlEvent::Characters(text) => {
                    if let Some(block) = stack.last_mut() {
                        block.raw_content = text.trim().to_string();
                    }
                }
                XmlEvent::EndElement { name } => {
                    if name.local_name != "UPF" {
                        if let Some(block) = stack.pop() {
                            if let Some(parent) = stack.last_mut() {
                                parent
                                    .sub_blocks
                                    .entry(name.local_name.clone())
                                    .or_insert_with(Vec::new)
                                    .push(block);
                            } else {
                                data.fields.insert(name.local_name, block);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_upf() {
        let input = r#"
<UPF version="2.0.1">
    <PP_HEADER element="Si" zp="4">
        <INFO>Some nested data</INFO>
    </PP_HEADER>
    <PP_R>
        0.0 1.1 2.2
    </PP_R>
</UPF>
        "#;

        let upf_data = UPFData::parse(input).unwrap();
        
        // Check version
        assert_eq!(upf_data.version, "2.0.1");

        // Check PP_HEADER
        let header = upf_data.fields.get("PP_HEADER").unwrap();
        assert_eq!(header.attributes["element"], "Si");
        assert_eq!(header.attributes["zp"], "4");

        // Check nested subfield INFO
        let info_blocks = &header.sub_blocks["INFO"];
        assert_eq!(info_blocks[0].raw_content.trim(), "Some nested data");

        // Check PP_R
        let pp_r = upf_data.fields.get("PP_R").unwrap();
        assert_eq!(pp_r.raw_content.trim(), "0.0 1.1 2.2");
    }
}