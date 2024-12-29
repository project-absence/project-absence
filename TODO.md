# TODO

This is a list of things I want to have done before releasing a v1.0.0

## Tool
- [ ] Have a `--version` CLI argument to get the current running version without running the tool
- [ ] Have a `help` command when running the tool, that will display the available modules
- [ ] UDP port scanner and make it a config of the `scanner:port` module
- [ ] Take a screenshot when a new hostname has been discovered -> https://crates.io/crates/headless_chrome
- [ ] Banner grabbing, e.g. getting the web server that is being ran
- [ ] Geolocation of newly discovered hostnames, though not sure if it's worth it at the moment
- [ ] Have a an option to allow only non-noisy modules to be ran. (e.g. enumeration modules that use wordlists won't be ran)
- [ ] Enumerate emails for newly discovered hostnames
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.
- [ ] Have a Docker image ready
- [ ] Features for each module? Not sure about that at the moment

## Documentation
- [ ] A page per module for their description, usage, examples, noise level and other things
- [ ] How to install the tool

## Website
- [ ] Have a MVP website where people can run the tool. As it's a sensible tool, some ownership proof of the domain will be needed (ideally a TXT record)
