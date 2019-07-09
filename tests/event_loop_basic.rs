use winit::event::*;
use winit::event_loop::*;

#[cfg(test)]
fn event_loop_test(flow: ControlFlow) {
    let event_loop = EventLoop::new();

    let mut first = true;
    let mut last = false;

    event_loop.run(move |e, _, ctl| {
        println!("event: {:?}", e);

        assert!(!last);
        *ctl = flow;
        match e {
            Event::NewEvents(StartCause::Init) => {
                assert!(first);
                first = false;
                *ctl = ControlFlow::Exit;
            }
            Event::LoopDestroyed => {
                assert!(!first);
                last = true;
            }
            _ => {
                assert!(!first);
            }
        }
    });
}
#[test]
fn event_loop_basic() {
    event_loop_test(ControlFlow::Poll);
}

#[test]
fn event_loop_wait() {
    event_loop_test(ControlFlow::Wait);
}
