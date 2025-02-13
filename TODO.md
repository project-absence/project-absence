# TODO

These are personal TODOs, if you'd like to contribute to the project, please do not work on these features; they are also a way for me to learn more things. Many many thanks :D

## Tool
- [ ] UDP port scanner and make it a config of the `scanner:port` module
- [ ] Geolocation of newly discovered hostnames
- [ ] Take a screenshot when a new hostname has been discovered -> https://crates.io/crates/headless_chrome
- [ ] Scripting engine -> https://crates.io/crates/mlua
- [ ] Google Dork module, find emails and other subdomains
- [ ] Generate a Markdown (later a Graph SVG and HTML as well) file for the results, not just the JSON view
- [ ] Reverse IP lookup to find other domains running on the same IP
- [ ] Have a web UI to run locally for the tool, will display running scans, etc.
- [ ] Enumerate emails for newly discovered hostnames
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.
- [ ] Trigger a webhook for the results, maybe provide support for specific webhooks e.g. Slack

## Website
- [ ] Roadmap of future goals (remove this file and put TODOs there as well?)
- [ ] Some place to login/signup and share config files or save the results of the tool on your personal account -> Maybe just a `community-resources` repository and letting people create PRs?
