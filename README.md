# Dioxus Audio Tag Event Bugs

This is a simple reproducible web app that demonstrates some issues with Dioxus's `audio` HTML tag event support.

* `trunk serve` from the root.
* navigate to http://http://127.0.0.1:8080/
* hit the play button

## Desired Behavior

When the audio tag loads metadata, updates time, or ends, we want the events to fire. In the sample app, this would show up in the HTML table that contains counts of each event.

## Actual Behavior

The events do not fire. The event counts do not increment in the HTML table.