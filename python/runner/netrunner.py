from dataclasses import dataclass
from deck import Cyberdeck


@dataclass
class NetrunnerAttributes:
    # maybe cool if each of these attributes had sub-skills
    # hacking devices, servers
    hacking: int = 5
    # quickhacking implants, usually for combat purposes
    quickhacking: int = 5
    # remaining undetected, masking your motives
    subtlety: int = 5
    # influencing people, social engineering
    charisma: int = 5
    # noticing information, trips, ...
    perception: int = 5

    def get_level(self, attribute_name: str):
        self.__getattribute__(attribute_name)


@dataclass
class NetrunnerResources:
    money: int = 100


class Netrunner:
    def __init__(self):
        self.attributes = NetrunnerAttributes()
        self.resources = NetrunnerResources()
        self.cyberdeck = Cyberdeck("babby's first deck")


if __name__ == "__main__":
    player = Netrunner()
    player.attributes.get_level("hacking")
