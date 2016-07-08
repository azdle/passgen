passgen
=======

[![Build Status](https://travis-ci.org/azdle/passgen.svg?branch=master)](https://travis-ci.org/azdle/passgen)
[![Crates.io Link](http://meritbadge.herokuapp.com/passgen)](https://crates.io/crates/passgen)

`passgen` is an incrdibly simple command line password generator. This initial version generates passwords of any length. Passwords are by default alphanumeric, but can optionally include special characters from an explicitly specified list.


Getting Started
---------------
The easiest way to use passgen is to install it through cargo.

```
$ cargo install passgen
```

Then generate a password:

```
$ passgen 40
Eu1UG65E4EWJaXUPr1jr0ZarssIMG1ktwM1CLwRW

$ passgen 40 :/!@#%^_-`
kgas4U5%bYRO#GoQH_Jn9Y%%z5V#ixQ9Ln2u^yMG

$ passgen 40 "():/!@#%^_-`*"
SGI*YY5P^07b@Q)SN*cc31FVxb2:4sWs)mfTN9g*
```
