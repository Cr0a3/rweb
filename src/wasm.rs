//! Wasm/html interop
use anyhow::bail;
use web_sys::Document;

/// Website metadata
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// The title
    pub title: Option<String>,
    /// Description
    pub description: Option<String>,
    /// Page keywords
    pub keywords: Vec<String>,
}

/// Sets the inner html of an element
pub fn set_inner_html(doc: &Document, id: &str, html: &str) {
    let element = doc.get_element_by_id(id).unwrap();
    element.set_inner_html(html);
}

impl Metadata {
    /// Applys the metadata to the document
    pub fn apply(&self, doc: &Document) -> anyhow::Result<()>  {
        let Some(head) = doc.head() else {
            bail!("Failed to retrive header element");
        };
        if let Some(title) = &self.title {
            doc.set_title(title);
        }
        if let Some(desc_str) = &self.description {
            let desc = doc.create_element("meta")?;
            desc.set_attribute("name", "description")?;
            desc.set_attribute("content", desc_str)?;
            head.append_child(&desc)?;
        }

        if !self.keywords.is_empty() {
            let item = doc.create_element("meta")?;
            item.set_attribute("name", "keywords")?;
            let mut string = String::new();
            for keyword in &self.keywords {
                string.push_str(keyword);
                string.push(',');
            }
            item.set_attribute("content",&string);
            head.append_child(&item)?;
        }

        Ok(())
    }
}
