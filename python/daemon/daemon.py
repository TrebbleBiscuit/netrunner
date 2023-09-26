class Death(Exception):
    ...


class Hitpoints:
    def __init__(self, max_hp: int, current_hp: int | None = None):
        self.max_hp = max_hp
        self._hp = current_hp or max_hp

    def __str__(self) -> str:
        return f"{self.hp} / {self.max_hp} HP"

    @property
    def is_dead(self) -> bool:
        if self.hp > 0:
            return False
        return True

    def take_damage(self, dmg: int):
        self.hp -= dmg

    @property
    def hp(self):
        return self._hp

    @hp.setter
    def hp(self, val):
        if val <= 0:
            print("oof that's death!")
            self._hp = 0
        elif val > self.max_hp:
            self._hp = self.max_hp
        else:
            self._hp = val


class RogueAI:
    def __init__(self):
        self.xp = 0
        self.hp = Hitpoints(20)

        # attributes
        self.hacking = 5
        self.firewall = 5


class Daemon:
    def __init__(self, hp: int = 10, power: int = 3, firewall: int = 2):
        self.hp = Hitpoints(hp)
        self.power = power  # offense
        self.firewall = firewall  # defense


class DaemonGame:
    def __init__(self):
        self.player = RogueAI()

    def hunt_daemons(self):
        print("you encounter an enemy daemon!")
        enemy = Daemon()
        still_fighting = True
        while still_fighting:
            print(f"You have {self.player.hp}")
            print(f"Enemy has {enemy.hp}")
            dmg_to_enemy = get_damage(self.player.hacking, enemy.firewall)
            print(f"You do {dmg_to_enemy} damage!")
            enemy.hp.take_damage(dmg_to_enemy)
            if enemy.hp.is_dead:
                break
            dmg_to_player = get_damage(enemy.power, self.player.firewall)
            print(f"You take {dmg_to_player} damage!")
            self.player.hp.take_damage(dmg_to_player)


def get_damage(atk_str: int, def_str: int) -> int:
    # each atk_str does 1 extra dmg
    # if def_str > atk_str, dmg is halved
    atk_val = atk_str + 1
    if def_str > atk_str:
        atk_val = atk_val // 2
    return atk_val


if __name__ == "__main__":
    game = DaemonGame()
    game.hunt_daemons()
