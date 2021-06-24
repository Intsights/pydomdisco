<p align="center">
    <a href="https://github.com/intsights/pydomdisco">
        <img src="https://raw.githubusercontent.com/intsights/pydomdisco/master/images/logo.png" alt="Logo">
    </a>
    <h3 align="center">
        A fast async domain discovery tool written in Rust
    </h3>
</p>


![license](https://img.shields.io/badge/MIT-License-blue)
![Python](https://img.shields.io/badge/Python-3.7%20%7C%203.8%20%7C%203.9-blue)
![OS](https://img.shields.io/badge/OS-Mac%20%7C%20Linux%20%7C%20Windows-blue)
![Build](https://github.com/intsights/pydomdisco/workflows/Build/badge.svg)
[![PyPi](https://img.shields.io/pypi/v/pydomdisco.svg)](https://pypi.org/project/pydomdisco/)

## Table of Contents

- [Table of Contents](#table-of-contents)
- [About The Project](#about-the-project)
  - [Built With](#built-with)
  - [Installation](#installation)
- [Usage](#usage)
- [License](#license)
- [Contact](#contact)


## About The Project

This library is intended to be used to discover registered domains according to the given TLD list by performing a fast and accurate resolving process. It was written in Rust in order to meet the performance requirements.


### Built With

* [pyo3](https://github.com/PyO3/pyo3)
* [tokio](https://github.com/tokio-rs/tokio)
* [trust-dns](https://github.com/bluejekyll/trust-dns)


### Installation

```sh
pip3 install pydomdisco
```


## Usage

```python
import pydomdisco


# Get a list of tlds to their corresponding nameservers IP addressed

# Only root tlds
root = pydomdisco.Discoverer.get_root_tld_to_nameservers_ips()

# Full list of the PSL tlds
psl = pydomdisco.Discoverer.get_psl_tld_to_nameservers_ips()

# Initialize the discovery engine loaded with the given tlds
discoverer = pydomdisco.Discoverer(root | psl)

# Perform the discovery process and return the list of discovered registered domains
registered_domains = discoverer.discover(
    [
        'google',
        'microsoft',
        'tesla',
    ],
)
```


## License

Distributed under the MIT License. See `LICENSE` for more information.


## Contact

Gal Ben David - gal@intsights.com

Project Link: [https://github.com/intsights/pydomdisco](https://github.com/intsights/pydomdisco)
