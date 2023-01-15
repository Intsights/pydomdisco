import importlib.resources
import pickle
import sys
import typing

from . import pydomdisco

PY_VERSION_MAJOR = sys.version_info.major
PY_VERSION_MINOR = sys.version_info.minor


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

    @staticmethod
    def pickle_load_pkl_file(
        file_name: str,
    ):
        if PY_VERSION_MAJOR == 3:
            if PY_VERSION_MINOR >= 9:
                with importlib.resources.files(
                    __package__,
                ).joinpath(
                    file_name,
                ).open(
                    'rb',
                ) as pkl_file:
                    return pickle.load(
                        file=pkl_file,
                    )
            elif PY_VERSION_MINOR == 8:
                with importlib.resources.open_binary(
                    package=__package__,
                    resource=file_name,
                ) as pkl_file:
                    return pickle.load(
                        file=pkl_file,
                    )
            elif PY_VERSION_MINOR <= 7:
                pkl_file = importlib.resources.open_binary(
                    package=__package__,
                    resource=file_name,
                )
                return pickle.load(
                    file=pkl_file,
                )
        raise RuntimeError(
            "Not supported python version",
        )

    @classmethod
    def get_root_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]:
        return cls.pickle_load_pkl_file(
            file_name='root_tld_to_nameservers_ips.pkl',
        )

    @classmethod
    def get_psl_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]:
        psl_tlds = cls.pickle_load_pkl_file(
            file_name='psl_tlds.pkl',
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
