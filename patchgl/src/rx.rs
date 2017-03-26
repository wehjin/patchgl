use std::cell::RefCell;

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
        let cell = RefCell::new(observer);
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
