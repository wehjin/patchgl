use std::error::Error;
use std::{thread, time};
use std::rc::Rc;

pub enum Reading<T> {
    Next(T),
    Complete,
    Error(Box<Error>),
}

pub trait Sequence<T>
{
    fn next(&mut self) -> Reading<T>;
    fn stop(&mut self);
}

pub trait Readable<T>
{
    fn sequence(&self) -> Box<Sequence<T>>;
    fn delay(&self, milliseconds: u64) -> BasicReadable<T>;
}

pub struct BasicSequence<T> {
    on_next: Box<FnMut() -> Reading<T>>,
    on_stop: Box<FnMut()>,
}

impl<T> Sequence<T> for BasicSequence<T> {
    fn next(&mut self) -> Reading<T> {
        (self.on_next)()
    }

    fn stop(&mut self) {
        (self.on_stop)()
    }
}

pub struct DelaySequence<T> {
    parent: Box<Sequence<T>>,
    should_delay: bool,
    milliseconds: u64,
}

impl<T> Sequence<T> for DelaySequence<T> {
    fn next(&mut self) -> Reading<T> {
        if self.should_delay {
            thread::sleep(time::Duration::from_millis(self.milliseconds));
            self.should_delay = false;
        }
        (self.parent).next()
    }

    fn stop(&mut self) {
        (self.parent).stop()
    }
}

pub struct BasicReadable<T>
{
    on_sequence: Rc<Box<Fn() -> Box<Sequence<T>>>>,
}

impl<T> Clone for BasicReadable<T> {
    fn clone(&self) -> Self {
        BasicReadable { on_sequence: self.on_sequence.clone() }
    }
}

impl<T> Readable<T> for BasicReadable<T> where T: 'static {
    fn sequence(&self) -> Box<Sequence<T>> {
        (self.on_sequence)()
    }
    fn delay(&self, milliseconds: u64) -> BasicReadable<T> {
        let parent = self.clone();
        from_on_sequence(Box::new(move || {
            Box::new(DelaySequence {
                parent: parent.sequence(),
                should_delay: true,
                milliseconds: milliseconds,
            })
        }))
    }
}

pub fn from_on_sequence<T>(on_sequence: Box<Fn() -> Box<Sequence<T>>>) -> BasicReadable<T> {
    BasicReadable {
        on_sequence: Rc::new(on_sequence),
    }
}

pub fn from_value<T>(value: T) -> BasicReadable<T> where T: Clone + 'static {
    from_on_sequence(Box::new(move || {
        let mut is_complete = false;
        let my_value = value.clone();
        let sequence = BasicSequence {
            on_next: Box::new(move || {
                if !is_complete {
                    is_complete = true;
                    Reading::Next(my_value.clone())
                } else {
                    Reading::Complete
                }
            }),
            on_stop: Box::new(|| {})
        };
        Box::new(sequence)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_readable_delivers_single_value() {
        let readable = from_value(35);
        let mut sequence = readable.sequence();
        match sequence.next() {
            Reading::Complete => assert!(false),
            Reading::Error(_) => assert!(false),
            Reading::Next(it) => assert_eq!(35, it),
        }
        match sequence.next() {
            Reading::Complete => (),
            Reading::Error(_) => assert!(false),
            Reading::Next(_) => assert!(false),
        }
    }

    #[test]
    fn lambda_readable_delivers_values_from_lambda() {
        let readable = from_on_sequence(Box::new(move || {
            let mut counter: u32 = 0;
            BasicSequence {
                on_next: Box::new(move || {
                    if counter == 0 {
                        counter = 1;
                        Reading::Next(0)
                    } else {
                        Reading::Complete
                    }
                }),
                on_stop: Box::new(|| {})
            }
        }));
        let mut sequence = readable.sequence();
        match sequence.next() {
            Reading::Complete => assert!(false),
            Reading::Error(_) => assert!(false),
            Reading::Next(it) => assert_eq!(0, it),
        }
        match sequence.next() {
            Reading::Complete => (),
            Reading::Error(_) => assert!(false),
            Reading::Next(_) => assert!(false),
        }
    }
}
