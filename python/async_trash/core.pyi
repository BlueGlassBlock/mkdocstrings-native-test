def sum_as_string(a: int, b: int) -> str:
    """Rust native function"""

async def async_add(a: int, b: int) -> int:
    """Rust native async function"""

class Adder:
    """Adder"""

    def __init__(self, param: int) -> None:
        """Init mess"""
    @property
    def param(self) -> int:
        """Rust native property"""
    def add(self, other: int) -> int:
        """Rust native method"""
    async def async_add(self, other: int) -> int:
        """Rust native async method"""
