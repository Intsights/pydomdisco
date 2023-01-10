import typing
import pickle
import importlib.resources

from . import pydomdisco


class Discoverer:
    def __init__(
        self,
        tld_to_nameservers_ips: typing.Dict[str, typing.List[str]],
        nameserver_timeout: float = 2.0,
    ) -> None:
        self.discoverer = pydomdisco.Discoverer(
            tld_to_nameservers_ips=tld_to_nameservers_ips,
            nameserver_timeout=nameserver_timeout,
        )

    def discover(
        self,
        names: typing.List[str],
        chunk_size: int = 20,
    ) -> typing.Dict[str, typing.List[str]]:
        return self.discoverer.discover(
            names=names,
            chunk_size=chunk_size,
        )

    @classmethod
    def get_root_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]:
        root_tld_to_nameservers_ips_file = importlib.resources.files(
            package=__package__,
        ).joinpath('root_tld_to_nameservers_ips.pkl').read_bytes()

        return pickle.load(
            file=root_tld_to_nameservers_ips_file,
        )

    @classmethod
    def get_psl_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]:
        psl_tlds_file = importlib.resources.files(
            package=__package__,
        ).joinpath('psl_tlds.pkl').read_bytes()

        psl_tlds = pickle.load(
            file=psl_tlds_file,
        )

        root_tld_to_nameservers_ips = cls.get_root_tld_to_nameservers_ips()

        psl_tlds = [
            tld
            for tld in psl_tlds
            if tld not in root_tld_to_nameservers_ips
        ]

        psl_tld_to_nameservers_ips = pydomdisco.Discoverer.generate_tld_to_nameservers_ips(
            tlds=psl_tlds,
        )

        return psl_tld_to_nameservers_ips
