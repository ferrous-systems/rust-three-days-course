# Teach Rust

"Teach Rust" is free workshop material to use to give a course introducing the Rust programming language. The time for the full course is around three to four days.

The material is created with people with zero Rust experience but with a programming background in mind.

## Commercial availability

This course is sponsored and trained by [asquera](http://asquera.de) in German and English. [Get in touch](mailto:info@asquera.de) for a custom quote.

The Spanish translation is community-maintained, we gladly refer to trainers.

## Overview

The course consists of two parts:

[The course presentations](presentation) is used as an introduction into all basic aspects of Rust. It comes in small parts and ships with integration into the Rust playground for examples as well with a translation structure.

[The work examples](example). The course constructs a small TCP server from ground up.

Along with that, all communication material we used is supplied [here](communication-material).

## The presentations

The presentation material is split into many small to medium presentations for every aspect of Rust. Not all are meant to fully cover a topic, but as an introduction of the important points. Chapters covering basic things such as ownership and borrowing should be exhaustive

The presentations are Reveal presentations with plugins for internationalisation and integration into the [Rust Playground](https://play.rust-lang.org).

The path through the presentations is not fixed, to allow leeway during holding the course. Examples of the pathes we used can be found in the [courses](courses) directory. We recommend to create your own while preparing for giving the workshop.

## The examples

Currently, the course ships with three examples:

* A simple TCP server that provides a very simple PUT and GET interface to store and remove messages
* The same built with Tokio and Futures
* A Rust library that can be used as a dynamic language

The TCP server is meant to be built from ground up, starting with a fresh crate.

The examples given here are an example of the final state. Before giving this course, you should develop this example by yourself.

## The Tasks

There are various tasks which can be found in `src/bin/` and can be run with `cargo run --bin $TASK"`.

The tasks are as follows:

* `add-chapter $CHAPTER` - Creates a chapter by the name of `$CHAPTER` for each locale.
* `add-code-example $CHAPTER $INDEX` - Appropriately renames all existing code examples (<=`$INDEX` are incremented by 1) for `$CHAPTER` to make room for a new code example. Also updates the slides to reflect this.
* `build-indices` - Builds the indices in `presentation/toc/` from `.md` to `html` for use in a web browser.
* `extract-code` - Extracts code samples from the slides in all chapters and creates appropriate copies in the `presentation/chapters/shared/code/*/*`, then updates the slides with new elements reflecting the new location, should not need to be regularly used.
* `rename-chapter $OLD $NEW` - Renames chapter `$OLD` to `$NEW` for all locales.
* `rename-chapters` - Renames chapters with an `md` extension to `chapter`. Should not need to be regularly used.

## Open issues

Currently, the largest issues are:

* the examples and the presentation lack READMEs

## Credits

The development of this course was financed by [Asquera](https://asquera.de) for the courses given at [Linuxhotel](https://linuxhotel.de).

They are open sourced as a contribution to the growth of the Rust language.

If you want to fund further development of the course, book a training!

## Commercial use

This course is expressively intended for commercial and free use.

## Trainers

* [Florian Gilcher](https://asquera.de)
* [Andrew Hobden](https://asquera.de)

Want to be on this list: open an [issue](https://github.com/skade/rust-three-days-course/issues) and we will add you.

## License

https://creativecommons.org/licenses/by-sa/4.0/

