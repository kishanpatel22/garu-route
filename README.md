# Garu-Route: GPS for Network Routes

🗺️ A network utility that traces the journey of your packets and pinpoints their physical location at every hop.

🚀 Why Garu-Route?

Standard traceroute tells you the hops your data takes. But if you are nerd, you need to know more. 
We need to know where those hops are. Has network packet left the country? Are they reaching the right data center? 
garu-route is built to answer these questions. 

✨ Features : 

* Enriches each hop with city, country, region, and precise coordinates.
* Metrics: Accurately measures the round-trip time (RTT) for every hop.
* Hostname Resolution: Resolves IP addresses to their corresponding hostnames.
* Clean Output: Displays all critical information in a simple, readable, and tabular format.

🖥️ Example

![image](https://i.giphy.com/2uIlaHVsql55CLP3as.webp)

Here's a live look at garu-route tracing the path to instagram.com from the place where I live :
```
❯ sudo ./target/debug/garu-route --domain instagram.com  
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| hop | ip_address      | hostname                            | duration    | city  | country | region | location        |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 1   | 192.178.29.1    | reliance.reliance                   | 5.132311ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 2   | 10.228.192.1    | ***                                 | 6.221353ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 3   | 172.16.5.10     | ***                                 | 6.561456ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 4   | 192.168.247.204 | ***                                 | 8.692783ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 5   | 192.168.230.213 | ***                                 | 6.48192ms   | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 6   | 192.168.230.194 | ***                                 | 7.589772ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 7   | 192.168.59.112  | ***                                 | 7.006649ms  | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 8   | ***             | ***                                 | 0ns         | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 9   | ***             | ***                                 | 0ns         | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 10  | 157.240.66.34   | ae13.pr03.del2.tfbnw.net            | 33.403687ms | Delhi | IN      | Delhi  | 28.6519,77.2315 |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 11  | 129.134.50.12   | po407.asw01.del2.tfbnw.net          | 30.940501ms | Delhi | IN      | Delhi  | 28.6519,77.2315 |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 12  | 129.134.94.176  | psw02.del2.tfbnw.net                | 32.602459ms | Delhi | IN      | Delhi  | 28.6519,77.2315 |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 13  | ***             | ***                                 | 0ns         | ***   | ***     | ***    | ***             |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
| 14  | 57.144.146.34   | instagram-p42-shv-03-del2.fbcdn.net | 74.670517ms | Delhi | IN      | Delhi  | 28.6519,77.2315 |
+-----+-----------------+-------------------------------------+-------------+-------+---------+--------+-----------------+
```

⚙️ Installation
From Source
```bash
$ git clone https://github.com/your-username/garu-route.git
$ cd garu-route
$ cargo build --release
```

🏃 UsageSince garu-route requires raw socket access to send and receive packets, it needs to be run with elevated privileges (sudo).
# Trace the path to a domain
```
$ sudo ./target/debug/garu-route --domain <domain_name>
```

🤝 Contributing
Contributions are welcome! If you have a feature request, bug report, or want to contribute to the code, please feel free to open an issue or pull request on the GitHub repository.
