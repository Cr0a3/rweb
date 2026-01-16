//! Wasm/html interop
use anyhow::{anyhow, bail};
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
    pub fn apply(&self, doc: &Document) -> anyhow::Result<()> {
        let Some(head) = doc.head() else {
            bail!("Failed to retrive header element");
        };
        if let Some(title) = &self.title {
            doc.set_title(title);
        }
        if let Some(desc_str) = &self.description {
            let desc = doc.create_element("meta").map_err(|e| anyhow!("{e:?}"))?;
            desc.set_attribute("name", "description")
                .map_err(|e| anyhow!("{e:?}"))?;
            desc.set_attribute("content", desc_str)
                .map_err(|e| anyhow!("{e:?}"))?;
            head.append_child(&desc).map_err(|e| anyhow!("{e:?}"))?;
        }

        if !self.keywords.is_empty() {
            let item = doc.create_element("meta").map_err(|e| anyhow!("{e:?}"))?;
            item.set_attribute("name", "keywords")
                .map_err(|e| anyhow!("{e:?}"))?;
            let mut string = String::new();
            for keyword in &self.keywords {
                string.push_str(keyword);
                string.push(',');
            }
            item.set_attribute("content", &string)
                .map_err(|e| anyhow!("{e:?}"))?;
            head.append_child(&item).map_err(|e| anyhow!("{e:?}"))?;
        }

        Ok(())
    }
}
