# domo-wot-bridge

[![Actions Status][actions badge]][actions]
[![CodeCov][codecov badge]][codecov]
[![LICENSE][license badge]][license]

DoMO component to control [Web of Things](https://www.w3.org/TR/wot-thing-description/)
 devices via the DoMO [DHT](https://github.com/domo-iot/domo-dht).

The SIFIS-HOME NSSD Manager is the SIFIS-HOME component responsible for interacting with the NSSD devices present in the house. It has been developed using the Rust language and is composed of three main modules: the DHT module, the M-DNS Module and the Web of Things (WoT)  Module.

- DHT Module: the DHT Module is the responsible for communicating with the DHT Manager. It uses the WebSocket API provided by the DHT Manager to access the DHT. In detail, it establishes a persistent WebSocket connection with the DHT Manager for being able to receive commands from the user (e.g. “turn on a certain light”) and for updating the status of the managed devices (e.g. to signal that an actuator is connected to the system).

- M-DNS Module: the M-DNS Module uses the m-DNS protocol to detect the presence of WiFi actuators in the network advertised by the DoMO gateway where it is in execution. In detail, the m-DNS module periodically performs an m-DNS discovery operation that produces as a result the list of WiFi actuators that are connected to the DoMO gateway advertised network.

- WoT Module: the Web of Things module manages the communication of the NSSD Manager with the NSSD. It uses a WoT API to interact with the NSSD.


# Acknowledgements

This software has been partially developed in the scope of the H2020 project SIFIS-Home with GA n. 952652.

<!-- Links -->
[actions]: https://github.com/domo-iot/domo-wot-bridge/actions
[codecov]: https://codecov.io/gh/domo-iot/domo-wot-bridge
[license]: LICENSE

<!-- Badges -->
[actions badge]: https://github.com/domo-iot/domo-wot-bridge/workflows/domo-wot-bridge/badge.svg
[codecov badge]: https://codecov.io/gh/domo-iot/domo-wot-bridge/branch/master/graph/badge.svg
[license badge]: https://img.shields.io/badge/license-MIT-blue.svg
