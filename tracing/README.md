# tracing

The core of `tracing`’s API is composed of *spans*, *events* and *subscribers*. We’ll cover these in turn.

tracing:

- spans
- events
- subscribers

## Spans

Unlike a log line that represents a *moment in time*, a span represents a *period of time* with a beginning and an end.
When a program begins executing in a context or performing a unit of work, it *enters* that context’s span, and when it
stops executing in that context, it *exits* the span. The span in which a thread is currently executing is referred to
as that thread’s *current* span.

```rust
use tracing::{Level, span};
fn main() {
    let span = span!(Level::TRACE, "hello");
    // `enter` returns a RAII guard which, when dropped, exits the span. this
    // indicates that we are in the span for the current lexical scope.
    let _enter = span.enter();
    println!("Hello, world!");
    // perform some work in the context of `my_span`...
}   // guard dropped here, span exited
```

- Level::TRACE means this span is only enabled if the subscriber (logger) is configured to record TRACE-level logs (the
  most verbose).
- "hello" is the name of the span. You can give it any descriptive string.

span!(...) creates a named context.
span.enter() makes that context active for the current scope.
The RAII guard ensures you leave the span when it drops.

> RAII stands for Resource Acquisition Is Initialization.
> RAII means owning an object = holding a resource,
> and when the object is dropped, the resource is cleaned up automatically.

## Events

An `Event` represents a _moment_ in time. an Event may occur within the context of a span.

```rust
use tracing::{event, span, Level};

// records an event outside of any span context:
event!(Level::INFO, "something happened");

let span = span!(Level::INFO, "my_span");
let _guard = span.enter();

// records an event within "my_span".
event!(Level::DEBUG, "something happened inside my_span");
```

In general, events should be used to represent points in time within a span — a request returned with a given status
code, n new items were taken from a queue, and so on.

## Subscribers

As Spans and Events occur, they are recorded or aggregated by implementations of the Subscriber trait. Subscribers are
notified when an Event takes place and when a Span is entered or exited. These notifications are represented by the
following Subscriber trait methods:

- event, called when an Event takes place,
- enter, called when execution enters a Span,
- exit, called when execution exits a Span

In addition, subscribers may implement the enabled function to filter the notifications they receive based on metadata
describing each Span or Event.

> We can filter the tracing event or span based on the metadata.

## Usage

The #[instrument] attribute provides an easy way to add tracing spans to functions. A function annotated
with #[instrument] will create and enter a span with that function’s name every time the function is called, with
arguments to that function will be recorded as fields using `fmt::Debug`.


shorthands for events: 

- trace!
- debug!
- info!
- warn!
- error!

shorthands for spans:

- trace_span!
- debug_span!
- info_span!
- warn_span!
- error_span!




