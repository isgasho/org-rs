//!  Plain Lists and Items
//! https://orgmode.org/worg/dev/org-syntax.html#Plain_Lists_and_Items
//!
//!  Items are defined by a line starting with the following pattern: “BULLET
//! COUNTER-SET CHECK-BOX TAG”, in which only BULLET is mandatory.
//!
//!  BULLET is either an asterisk, a hyphen, a plus sign character or follows
//! either the pattern “COUNTER.” or “COUNTER)”.  In any case, BULLET is follwed by
//! a whitespace character or line ending.
//!
//!  COUNTER can be a number or a single letter.
//!
//!  COUNTER-SET follows the pattern [@COUNTER].
//!
//!  CHECK-BOX is either a single whitespace character, a “X” character or a
//! hyphen, enclosed within square brackets.
//!
//!  TAG follows “TAG-TEXT ::” pattern, where TAG-TEXT can contain any character
//! but a new line.
//!
//!  An item ends before the next item, the first line less or equally indented
//! than its starting line, or two consecutive empty lines. Indentation of lines
//! within other greater elements do not count, neither do inlinetasks boundaries.
//!
//!  A plain list is a set of consecutive items of the same indentation. It can
//! only directly contain items.
//!
//!  If first item in a plain list has a counter in its bullet, the plain list will
//! be an “ordered plain-list”. If it contains a tag, it will be a “descriptive
//! list”. Otherwise, it will be an “unordered list”. List types are mutually
//! exclusive.
//!
//!  For example, consider the following excerpt of an Org document:
//!
//!  1. item 1
//!  2. [X] item 2
//!     - some tag :: item 2.1
//!
//!
//!  Its internal structure is as follows:
//!
//!
//!  (ordered-plain-list
//!   (item)
//!   (item
//!    (descriptive-plain-list
//!     (item))))
//!

use std::borrow::Cow;

/// List structure
/// This looks like an intermediate list representation, required both by
/// plain list itself and items in the list.
pub struct ListStruct {
    // stub
}

pub struct ItemData<'rope> {
    /// Item's bullet (string).
    bullet: Cow<'rope, str>,
    /// Item's check_box, if any (symbol on, off, trans, nil).
    checkbox: Option<CheckBox>,
    /// Item's counter, if any. Literal counters become ordinals (integer).
    counter: usize,
    /// Number of newline characters between the beginning
    /// of the item and the beginning of the contents (0, 1 or 2).
    pre_blank: usize,
    /// Uninterpreted item's tag, if any (string or nil).
    raw_tag: Option<Cow<'rope, str>>,
    /// Parsed item's tag, if any (secondary string or nil).
    tag: Option<Cow<'rope, str>>,
    // TODO figure out what is list structure
    // /// Full list's structure, as returned by org_list_struct (alist).
    structure: ListStruct,
}

pub struct PlainListData {
    /// Full list's structure, as returned by org_list_struct (alist).
    pub structure: ListStruct,

    ///List's type (symbol descriptive, ordered, unordered).
    pub type_s: ListKind,
}

pub enum ListKind {
    Descriptive,
    Ordered,
    Unordered,
}

pub enum CheckBox {
    On,
    Off,
    Trans,
}
