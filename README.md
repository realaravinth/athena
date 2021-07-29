<div align="center">
<h1> Athena </h1>
<p>

**HTTP based framework for building
[C2](https://en.wikipedia.org/wiki/Command_and_control) for
[RATs](https://en.wikipedia.org/?title=Remote_access_trojan)**

</p>

[![Documentation](https://img.shields.io/badge/docs-libathena_master-blue)](https://realaravinth.github.io/athena/libathena/)
[![Build](https://github.com/realaravinth/athena/actions/workflows/linux.yml/badge.svg)](https://github.com/realaravinth/athena/actions/workflows/linux.yml)
[![dependency status](https://deps.rs/repo/github/realaravinth/athena/status.svg)](https://deps.rs/repo/github/realaravinth/athena)
[![codecov](https://codecov.io/gh/realaravinth/athena/branch/master/graph/badge.svg)](https://codecov.io/gh/realaravinth/athena)

</div>

**Disclaimer: This software is not authorized for use in committing
computer fraud. The authors of this software CAN NOT be held responsible
for the program's users' actions**

Athena provides a simple, user-friendly API to compose transports for RATs.

Establishing bind shells with victims behind firewalls is a difficult
task. Athena provides an HTTP API for both attacking and victim
components. 

An interactive bind shell the attacker can poll the API in regular intervals.

## Components

| Component   | Documentation                                                                                                       | Description                                               |
| ----------- | ------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------- |
| `libathena` | [![Documentation](https://img.shields.io/badge/docs-master-blue)](https://realaravinth.github.io/athena/libathena/) | Client library for attacker and victim components of RATs |
| `athena-c2` | Work in progress                                                                                                    | C2 server for Athena                                      |
