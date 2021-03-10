from dataclasses import dataclass


@dataclass
class Property:
    key: str
    value: str
    pattern: str = r'()[]'
