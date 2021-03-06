# Changelog

## v0.2.1 - now

## Added

* Add `AlternateScreenBackend` in `termion` backend
* Add `TermionBackend::with_stdout` in order to let an user of the library
provides its own termion struct

## v0.2.0 - 2017-12-26

### Added

* Add `MouseBackend` in `termion` backend to handle scroll and mouse events
* Add generic `Item` for items in a `List`

### Changed

* Rename `TermionBackend` to `RawBackend` (to distinguish it from the `MouseBackend`)
* Generic parameters for `List` to allow passing iterators as items
* Generic parameters for `Table` to allow using iterators as rows and header
* Generic parameters for `Tabs`
* Rename `border` bitflags to `Borders`

* Run latest `rustfmt` on all sources

### Removed

* Drop `log4rs` as a dev-dependencies in favor of `stderrlog`
