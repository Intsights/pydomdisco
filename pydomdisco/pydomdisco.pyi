import typing


class Discoverer:
    def __init__(
        self,
        tld_to_nameservers_ips: typing.Dict[str, typing.List[str]],
        nameserver_timeout: float = 2.0,
    ) -> None: ...

    def discover(
        self,
        names: typing.List[str],
        chunk_size: int = 20,
    ) -> typing.Dict[str, typing.List[str]]: ...

    @classmethod
    def get_root_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]: ...

    @classmethod
    def get_psl_tld_to_nameservers_ips(
        cls,
    ) -> typing.Dict[str, typing.List[str]]: ...
