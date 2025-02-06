# TODO

## Tool
- [ ] Rework the random user agents, make a static vec with some values and then a function to get a random one
- [ ] Remove `flume` dependency and just use `mpsc` from `std::sync` -> https://doc.rust-lang.org/std/sync/mpsc
- [ ] UDP port scanner and make it a config of the `scanner:port` module
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.
- [ ] Geolocation of newly discovered hostnames, though not sure if it's worth it at the moment
- [ ] Take a screenshot when a new hostname has been discovered -> https://crates.io/crates/headless_chrome
- [ ] Enumerate emails for newly discovered hostnames
- [ ] Trigger a webhook for the results, maybe provide support for specific webhooks e.g. Slack

## Website
- [ ] Roadmap of future goals?
- [ ] Some place to login/signup and share config files or save the results of the tool on your personal account
