# TODO

This is a list of things I want to have done before releasing a v1.0.0

## Tool
- [ ] UDP port scanner and make it a config of the `scanner:port` module
- [ ] Take a screenshot when a new hostname has been discovered -> https://crates.io/crates/headless_chrome
- [ ] Banner grabbing, e.g. getting the web server that is being ran
- [ ] Have a an option to allow only non-noisy modules to be ran. (e.g. enumeration modules that use wordlists won't be ran)
- [ ] Enumerate emails for newly discovered hostnames
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.
- [ ] Have a Docker image ready
- [ ] Geolocation of newly discovered hostnames, though not sure if it's worth it at the moment
- [X] Have the tool report some statistics while running (e.g. memory usage, ~~cpu usage~~ & total tasks/threads running)
- [X] Make the copy to clipboard functionality behind a feature to prevent being forced to install the required packages by the `clipboard` crate on Linux systems
- [X] Write a proper `README.md` file with its sections
- [X] Have a `--version` CLI argument to get the current running version without running the tool

## Documentation
- [ ] A page per module for their description, usage, examples, noise level and other things
- [ ] How to install the tool

## Website
- [ ] Have a MVP website where people can run the tool. As it's a sensible tool, some ownership proof of the domain will be needed (ideally a TXT record)
