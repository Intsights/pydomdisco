import unittest
import pydomdisco


class PyDomDiscoTestCase(
    unittest.TestCase,
):
    def test_discoverer(
        self,
    ):
        root_tlds = pydomdisco.Discoverer.get_root_tld_to_nameservers_ips()
        discoverer = pydomdisco.Discoverer(root_tlds)
        discovered_domains = discoverer.discover(
            names=[
                'google',
            ],
        )

        self.assertGreater(
            a=len(discovered_domains),
            b=100,
        )
