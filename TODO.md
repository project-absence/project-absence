# TODO

These are personal TODOs, if you'd like to contribute to the project, please do not work on these features; they are also a way for me to learn more things. Many many thanks :D

## Tool
- [ ] Allow to overwrite the CLI arguments in the config file under `[args]` or `[core]`
- [ ] Trigger a webhook for the results, maybe provide support for specific webhooks e.g. Slack
- [ ] Have a web UI to run locally for the tool, will display running scans, etc.
- [ ] Have a core command like now that takes the config file, but also have seperate commands and its relevant configuration, e.g. `project-absence --domain <your-domain> dork --search-engine ecosia`
  - This will likely require some cleanup in the modules themselves and instead of returning `Result<(), String>` when executing a module, it would return the (list?) of `events::Type` to emit, or keep it as now but have a seperate crate or module to perform the actions, like check a port
- [ ] Support more search engines and dorking methods
- [ ] Enumerate emails for newly discovered domains
- [ ] UDP port scanner and make it a config of the `scanner:port` module
- [ ] Overall SSL scanner -> Is the certificate trusted, is it vulnerable to Heartbleed, etc.

## Website
- [ ] Roadmap of future goals (remove this file and put TODOs there as well?)
