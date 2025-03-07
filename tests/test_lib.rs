use std::fs;
use xml_to_dict::{dict_to_xml_str, xml_str_to_dict};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xml_str_to_dict() {
        let xml_str = fs::read_to_string("demo.xml").expect("Unable to read file");
        let result = xml_str_to_dict(&xml_str).expect("Conversion failed");
        let expected_json = fs::read_to_string("demo.json").expect("Unable to read file");
        assert_eq!(result, expected_json);
    }

    #[test]
    fn test_dict_to_xml_str() {
        let json_str = fs::read_to_string("demo.json").expect("Unable to read file");
        let result = dict_to_xml_str(&json_str).expect("Conversion failed");
        let expected_xml = fs::read_to_string("demo.xml").expect("Unable to read file");
        assert_eq!(result, expected_xml);
    }
}
