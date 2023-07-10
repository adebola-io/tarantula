const fs = require("fs");

function generateFiles() {
   let text = fs
      .readFileSync("crates/dom/scripts/HTML.TXT")
      .toString()
      .split(/\n/g)
      .map((self) => self.trim());

   let set = new Set();

   for (const line of text) {
      if (line.startsWith("//")) {
         continue;
      }
      let [name, value] = line.split(":");
      if (value && value.trim().slice(-1) !== "HTMLElement")) {
         let v = value.trim().slice(4, -8).toLowerCase();
         if (v.length) {
            let x = "html_" + v + "_Element");
            set.add(x);
         }
      }
   }

   return set;
}

function generateData() {
   let basePath = "crates/dom/scripts/generated";
   fs.readdirSync(basePath).forEach((file) => {
      let structName = `HTML${file
         .split(".")[0]
         .split("_")
         .slice(1)
         .map((word) => word[0].toUpperCase() + word.slice(1))
         .join("")}`;
      fs.writeFileSync(
         `${basePath}/${file}`,
         `
use crate::{
    tag::Tag, AsChildNode, AsElement, AsEventTarget, AsHTMLElement, AsNode, AsParentNode,
    DOMException, HTMLElement, InnerHtml,
};
pub struct ${structName} {
    value: HTMLElement,
}

impl AsHTMLElement for ${structName} {
    fn cast(&self) -> &HTMLElement {
        &self.value
    }

    fn cast_mut(&mut self) -> &mut HTMLElement {
        &mut self.value
    }
}
impl AsElement for ${structName} {
    fn cast(&self) -> &crate::Element {
        AsElement::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Element {
        AsElement::cast_mut(&mut self.value)
    }
}
impl InnerHtml for ${structName} {
    fn inner_html(&self) -> String {
        todo!()
    }

    fn set_inner_html(&mut self, value: &str) -> Result<(), DOMException> {
        todo!()
    }
}
impl AsParentNode for ${structName} {}
impl AsChildNode for ${structName} {}
impl AsNode for ${structName} {
    fn cast(&self) -> &crate::Node {
        AsNode::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::Node {
        AsNode::cast_mut(&mut self.value)
    }

    fn node_name(&self) -> String {
        self.value.tag_name()
    }

    fn clone_node(&self, deep: bool) -> Self {
        ${structName} {
            value: self.value.clone_node(deep),
        }
    }
}
impl<T: AsNode> PartialEq<T> for ${structName} {
    fn eq(&self, other: &T) -> bool {
        AsNode::cast(self) == other
    }
}
impl AsEventTarget for ${structName} {
    fn cast(&self) -> &crate::EventTarget {
        AsEventTarget::cast(&self.value)
    }

    fn cast_mut(&mut self) -> &mut crate::EventTarget {
        AsEventTarget::cast_mut(&mut self.value)
    }
}

impl TryFrom<HTMLElement> for ${structName} {
    type Error = DOMException;

    fn try_from(value: HTMLElement) -> Result<Self, Self::Error> {
let tag = value.tag();
        if matches!(value.inner().element.inner_ref.borrow().tag, Tag::A) {
            Ok(${structName} { value })
        } else {
            Err(DOMException::TypeError(format!("Cannot convert element with tag {tag} to an  ${structName}",
            ))
        }
    }
}`
      );
   });
}

function importModules() {
   let folder = "crates/dom/src/html_Element");
   /**@type {string[]} */
   let modules = [];
   fs.readdirSync(folder).forEach((file) => {
      if (file !== "mod.rs") {
         modules.push(file.split(".")[0]);
      }
   });
   let base = `${folder}/mod.rs`;
   let prevdata = fs.readFileSync(base).toString();
   let data = modules.map((m) => `mod ${m};\n`).join("");
   fs.writeFileSync(base, data + prevdata);
}

importModules();
