use crate::renderer::dom::node::Node;
use crate::renderer::dom::node::Window;
use crate::renderer::html::token::HtmlTokenizer;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

/// https://html.spec.whatwg.org/multipage/parsing.html#the-insertion-mode
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum InsertionMode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    AfterHead,
    InBody,
    Text,
    AfterBody,
    AfterAfterBody,
}

#[derive(Debug, Clone)]
pub struct HtmlParser {
    window: Rc<RefCell<Window>>,
    mode: InsertionMode,
    original_insertion_mode: InsertionMode,
    stack_of_open_elements: Vec<Rc<RefCell<Node>>>,
    t: HtmlTokenizer,
}

impl HtmlParser {
}