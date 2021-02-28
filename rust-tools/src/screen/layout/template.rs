use std::collections::HashMap;
use std::rc::Rc;

use screen::layout::layout::{L, LayoutRc};
use screen::layout::str_layout::{StrLayout, StrPtr};

pub struct Template {
    pub template: LayoutRc,
    pub vars: HashMap<usize, StrPtr>,
}

impl Template {
    pub fn empty() -> Template {
        Template::new(L::str(""))
    }
    pub fn new(template: LayoutRc) -> Template {
        Template {
            template,
            vars: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: usize, var: &StrPtr) {
        self.vars.insert(key, var.clone());
    }

    pub fn update(&mut self, key: usize, value: &str) {
        match self.vars.get_mut(&key) {
            None => panic!(),
            Some(layout) => {
                layout.update(value);
            }
        }
    }
}

#[test]
fn test_template() {
    let x = L::str("coucou");
    let mut template = Template::new(
        L::vert(vec![x.clone(), x.clone()]));
    template.register(0, &x);


    template.template.show();
    template.update(0, "toto");
    template.template.show();
}
