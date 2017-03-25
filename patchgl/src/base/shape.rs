use super::Typeface;
use super::Alignment;

pub enum Shape {
    Rectangle,
    Word(String, Typeface, Alignment)
}