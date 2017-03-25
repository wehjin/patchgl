#[macro_use]
extern crate glium;
extern crate xml;
extern crate cage;
extern crate rusttype;
extern crate unicode_normalization;
extern crate arrayvec;

pub mod parser;
pub mod model;
pub mod renderer;
pub mod glyffin;
pub mod screen;
pub mod base;

use model::Patchwork;
use renderer::PatchRenderer;
use glyffin::QuipRenderer;
use rusttype::{Scale};
use glium::glutin;
use screen::Screen;
use glium::{Surface};
use glium::glutin::Event;

pub trait Observer<Value, Error> {
    fn on_next(&mut self, value: Value);
    fn on_complete(&mut self, );
    fn on_error(&mut self, error: Error);
}

pub trait Subscription {
    fn unsubscribe(&mut self);
    fn is_unsubscribed(&self) -> bool;
}

pub struct BooleanSubscription
{
    is_unsubscribed: bool,
    on_unsubscribe: Box<Fn()>,
}

impl Subscription for BooleanSubscription
{
    fn unsubscribe(&mut self) {
        if self.is_unsubscribed {
            return
        }
        (self.on_unsubscribe)();
        self.is_unsubscribed = true;
    }

    fn is_unsubscribed(&self) -> bool {
        self.is_unsubscribed
    }
}

pub trait Observable<Value, Error> {
    fn subscribe(&self, observer: Box<Observer<Value, Error>>) -> Box<Subscription>;
}

struct RootObservable<Value, Error>
{
    on_subscribe: Box<Fn(&Fn(Value), &Fn(Error))>
}

impl<Value, Error> Observable<Value, Error> for RootObservable<Value, Error>
    where Value: 'static, Error: 'static
{
    fn subscribe(&self, observer: Box<Observer<Value, Error>>) -> Box<Subscription> {
        let subscription = BooleanSubscription {
            is_unsubscribed: false,
            on_unsubscribe: Box::new(|| {}),
        };
        let cell = std::cell::RefCell::new(observer);
        (self.on_subscribe)(&|value| {
            if subscription.is_unsubscribed {
                return
            }
            cell.borrow_mut().on_next(value)
        }, &|error| {
            if subscription.is_unsubscribed {
                return
            }
            cell.borrow_mut().on_error(error)
        });
        Box::new(subscription)
    }
}

pub fn create_observable<Value, Error, F>(on_subscribe: F) -> Box<Observable<Value, Error>>
    where Value: 'static, Error: 'static, F: Fn(&Fn(Value), &Fn(Error)) + 'static
{
    let observable = RootObservable { on_subscribe: Box::new(on_subscribe) };
    Box::new(observable)
}

pub fn go() {
    let xml = include_str!("screen_with_square_patch.xml");
    let patchwork = Patchwork::from_xml(xml);

    let screen = Screen::new(320, 480);
    let display = &screen.display;

    let patch_renderer = PatchRenderer::new(&patchwork, &display);
    let modelview = patch_renderer.get_modelview(&display);

    let mut quip_renderer = QuipRenderer::new(screen.dpi_factor(), modelview, display);
    quip_renderer.layout_paragraph("I for one welcome our new robot overlords",
                                   Scale::uniform(24.0 * screen.dpi_factor()), screen.width, display);

    loop {
        let mut target = display.draw();
        target.clear_color(0.70, 0.80, 0.90, 1.0);

        patch_renderer.draw(&mut target, &display);
        quip_renderer.draw(&mut target);

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) | glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}

#[cfg(test)]
mod tests {}