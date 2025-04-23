use std::{collections::HashMap, sync::LazyLock};

/// Uses https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers
/// Skipping the "Dynamic, private or ephemeral ports" section
/// The list has the most commont ports and their related service most likely running behind it.
///
/// More ports and services will be added over time
/// The ports like 5000, 7777 or 8000 are explicitly ignored, because there is like 4+ services that can be running behind it.
static SERVICE_FOR_TCP_PORT: LazyLock<HashMap<usize, String>> = LazyLock::new(|| {
    let mut map = HashMap::<usize, String>::new();
    map.insert(21, "ftp".to_string());
    map.insert(22, "ssh".to_string());
    map.insert(23, "telnet".to_string());
    map.insert(25, "smtp".to_string());
    map.insert(43, "whois".to_string());
    map.insert(53, "dns".to_string());
    map.insert(70, "gopher".to_string());
    map.insert(80, "http".to_string());
    map.insert(88, "kerberos-auth".to_string());
    map.insert(110, "pop3".to_string());
    map.insert(115, "sftp".to_string());
    map.insert(143, "imap".to_string());
    map.insert(389, "ldap".to_string());
    map.insert(443, "https".to_string());
    map.insert(464, "kerberos-password".to_string());
    map.insert(631, "cups".to_string());
    map.insert(749, "kerberos-admin".to_string());
    map.insert(990, "ftps".to_string());
    map.insert(993, "imaps".to_string());
    map.insert(995, "pop3s".to_string());
    map.insert(1194, "openvpn".to_string());
    map.insert(1476, "wifi-pineapple".to_string());
    map.insert(2082, "cpanel".to_string());
    map.insert(2456, "valheim".to_string());
    map.insert(3306, "mysql".to_string());
    map.insert(3389, "rdp".to_string());
    map.insert(4200, "angular".to_string());
    map.insert(4444, "metasploit".to_string());
    map.insert(5432, "postgresql".to_string());
    map.insert(5900, "vnc".to_string());
    map.insert(6379, "redis".to_string());
    map.insert(6660, "irc".to_string());
    map.insert(6661, "irc".to_string());
    map.insert(6662, "irc".to_string());
    map.insert(6663, "irc".to_string());
    map.insert(6664, "irc".to_string());
    map.insert(6665, "irc".to_string());
    map.insert(6666, "irc".to_string());
    map.insert(6667, "irc".to_string());
    map.insert(6668, "irc".to_string());
    map.insert(6669, "irc".to_string());
    map.insert(8080, "http-proxy".to_string());
    map.insert(8443, "https".to_string());
    map.insert(25565, "minecraft".to_string());
    map
});

pub fn get_service_for_tcp_port(port: usize) -> String {
    match SERVICE_FOR_TCP_PORT.get(&port) {
        Some(service) => service.to_string(),
        None => "unknown".to_string(),
    }
}
