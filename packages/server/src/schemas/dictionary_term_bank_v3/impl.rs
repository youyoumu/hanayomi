use super::*;

impl DictionaryTermBankV3Row {
    pub fn test(&self) {
        let item = self.5.first().unwrap();
        match item {
            Definition::Text(_) => {
                println!("DEBUG[1410]: this is String");
            }
            Definition::Detailed(definition) => match definition.as_ref() {
                DetailedDefinition::Text(_) => {
                    println!("DEBUG[1410]: this is Text");
                }
                DetailedDefinition::Image(_) => {
                    println!("DEBUG[1410]: this is Image");
                }
                DetailedDefinition::StructuredContent(sc) => {
                    test_structured_content(sc.content.as_ref());
                    // println!("DEBUG[1410]: this is StructuredContent");
                }
            },
            Definition::Deinflection(_) => {
                println!("DEBUG[1410]: this is Array");
            }
        }
    }
}

fn test_structured_content(content: &StructuredContent) {
    match content {
        StructuredContent::Text(_) => {
            // println!("DEBUG[1410]: this is StructuredContentText");
        }
        StructuredContent::Array(arr) => {
            // println!("DEBUG[1410]: this is StructuredContentArray");
            for sc in arr.iter() {
                test_structured_content(sc);
            }
        }
        StructuredContent::Object(obj) => {
            test_structured_content_object(obj);
            // println!("DEBUG[1410]: this is StructuredContentObject");
        }
    }
}

fn test_structured_content_object(obj: &StructuredContentObject) {
    match obj {
        StructuredContentObject::Br(_) => {
            println!("DEBUG[1410]: this is StructuredContentBr ");
        }
        StructuredContentObject::Ruby(_) => {
            println!("DEBUG[1410]: this is StructuredContentRuby ");
        }
        StructuredContentObject::Rt(_) => {
            println!("DEBUG[1410]: this is StructuredContentRt ");
        }
        StructuredContentObject::Rp(_) => {
            println!("DEBUG[1410]: this is StructuredContentRp ");
        }
        StructuredContentObject::Table(_) => {
            println!("DEBUG[1410]: this is StructuredContentTable ");
        }
        StructuredContentObject::Thead(_) => {
            println!("DEBUG[1410]: this is StructuredContentThead ");
        }
        StructuredContentObject::Tbody(_) => {
            println!("DEBUG[1410]: this is StructuredContentTbody ");
        }
        StructuredContentObject::Tfoot(_) => {
            println!("DEBUG[1410]: this is StructuredContentTfoot ");
        }
        StructuredContentObject::Tr(_) => {
            println!("DEBUG[1410]: this is StructuredContentTr ");
        }
        StructuredContentObject::Td(_) => {
            println!("DEBUG[1410]: this is StructuredContentTd ");
        }
        StructuredContentObject::Th(_) => {
            println!("DEBUG[1410]: this is StructuredContentTh ");
        }
        StructuredContentObject::Span(fields) => {
            println!("DEBUG[1410]: this is StructuredContentSpan ");
            if let Some(content) = fields.content.as_ref() {
                test_structured_content(content)
            }
        }
        StructuredContentObject::Div(fields) => {
            if let Some(content) = fields.content.as_ref() {
                test_structured_content(content)
            }
            println!("DEBUG[1410]: this is StructuredContentDiv");
        }
        StructuredContentObject::Ol(_) => {
            println!("DEBUG[1410]: this is StructuredContentOl ");
        }
        StructuredContentObject::Ul(_) => {
            println!("DEBUG[1410]: this is StructuredContentUl ");
        }
        StructuredContentObject::Li(_) => {
            println!("DEBUG[1410]: this is StructuredContentLi ");
        }
        StructuredContentObject::Details(_) => {
            println!("DEBUG[1410]: this is StructuredContentDetails ");
        }
        StructuredContentObject::Summary(_) => {
            println!("DEBUG[1410]: this is StructuredContentSummary ");
        }
        StructuredContentObject::Img(_) => {
            println!("DEBUG[1410]: this is StructuredContentImg ");
        }
        StructuredContentObject::A(_) => {
            println!("DEBUG[1410]: this is StructuredContentA ");
        }
    }
}
