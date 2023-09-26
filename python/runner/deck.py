from dataclasses import dataclass, field


@dataclass
class Quickhack:
    ...


class CyberdeckSlots:
    def __init__(self, num_slots: int = 3):
        self.number_slots = num_slots
        # divide into attack/defense/utility slots?
        self.slot_contents: list[Quickhack | None] = [None for _ in range(num_slots)]

    def set_slot(self, slot_number: int, new_contents: Quickhack | None):
        if slot_number < 0:
            raise ValueError
        if slot_number >= self.number_slots:
            raise ValueError("Not enough available slots")
        self.slot_contents[slot_number] = new_contents


@dataclass
class CyberdeckStats:
    # process data and perform actions faster
    speed: int = 5
    # do more actions at once
    memory: int = 5


class Cyberdeck:
    def __init__(self, name: str):
        self.name: str = name
        self.slots: CyberdeckSlots = CyberdeckSlots()
        self.stats: CyberdeckStats = CyberdeckStats()
