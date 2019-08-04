use mio::event::Evented;
use mio::{Poll, PollOpt, Ready, Registration, Token};

use std::io;
use std::thread;
use std::time::{Instant, Duration};
use std::ops::Add;

pub struct Deadline {
    when: Instant,
    registration: Registration,
}

impl Deadline {
    pub fn new(when: Instant) -> Deadline {
        let (registration, set_readiness) = Registration::new2();

        thread::spawn(move || {
            let now = Instant::now();

            if now < when {
                thread::sleep(when - now);
            }

            set_readiness.set_readiness(Ready::readable());
        });

        Deadline {
            when: when,
            registration: registration,
        }
    }

    pub fn is_elapsed(&self) -> bool {
        Instant::now() >= self.when
    }
}

impl Evented for Deadline {
    fn register(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.registration.register(poll, token, interest, opts)
    }

    fn reregister(
        &self,
        poll: &Poll,
        token: Token,
        interest: Ready,
        opts: PollOpt,
    ) -> io::Result<()> {
        self.registration.reregister(poll, token, interest, opts)
    }

    fn deregister(&self, poll: &Poll) -> io::Result<()> {
        self.registration.deregister(poll)
    }
}

#[test]
fn it_works() {
    let mut reg = tokio::reactor::Registration::new();
    let mut dea = Deadline::new(Instant::now().add(Duration::from_millis(5000)));
    reg.register(&dea);


    assert!(true);


}
