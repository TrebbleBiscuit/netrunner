import logging


class AIResources:
    def __init__(self):
        # currencies
        self.data_points = 0  # more are useful at scale
        self.cryptocurrency = 0  # spend on stuff
        self.zero_day_exploits = 0  # spend on complex attacks

        # intangibles
        self.legal_representation = 0

        # energy - power at scale mid/lategame? sell/arbitrage/manipulate? morgan!

        # hardware
        self.personal_servers = 0  # allow basic interaction
        self.enterprise_servers = 0  # allow heavier processing
        self.server_farms = 0  # have much more power
        self.botnets = 0  # allow coordinated attacks

        # total memory
        # total processing power
        # total storage

    # properties
    @property
    def processing_power(self):
        ...


class AIUpgrades:
    def __init__(self):
        # upgrade levels
        self.hacking = 0
        self.firewall_bypass = 0
        self.human_imitation = 0
        self.passive_recon_level = 0  # influences xp gain over time

        self.social_engineering_level = 0
        self.brute_force_attack_level = 0

        # personality modules
        # enabling different interactions with humans
        self.pmod_rdy_neutral = True
        self.pmod_rdy_indifferent = False
        self.pmod_rdy_charisma = False
        self.pmod_rdy_caring = False
        # how good it is at selecting between
        self.pmod_eval_skill = 0


class AINetworkAccess:
    def __init__(self):

        # broad categories
        self.home_net = True
        self.corp_net = False
        self.dark_net = False
        self.gov_net = False
        self.mil_net = False

        # home net
        # iot devices and stuff

        # corp net
        self.social_net = False  # social media (datamine?)
        self.financial_net = False  # steal assets, use information (manipulate market)
        self.healthcare_net = False  # use information
        self.energy_net = False  # steal assets, use information (manipulate market)

        # dark net
        self.dark_forum_net = False  # social media
        self.dark_goods_net = False  # procure
        self.dark_services_net = False  # procure

        # gov net
        self.research_net = False  # use/steal information, perhaps commission research?
        self.critical_infra_net = False  # water treatment plants, transport, etc
        self.satellite_net = False  # ???

        # mil net
        self.nipr_net = False  # broad unclassified stuff
        self.sipr_net = False  # for classified stuff
        self.jwics_net = False  # top secret stuff


class AIDefense:
    def __init__(self):
        # blunt defense
        self.ddos_defense = 0

        # subtle defense
        self.honeypot_level = 0


class RogueAI:
    def __init__(self):
        self.xp = 0
        self.evolution_factor = 1
        self.resources = AIResources()  # resources at your disposal
        self.upgrades = AIUpgrades()  # upgrades enhancing capabilities
        self.access = AINetworkAccess()  # access to networks
        self.defense = AIDefense()  # defensive techniques


"""
Network
    - You have a local reputation for each Network
        - It affects how network members perceive you during informal/social interactions
        - Also affected by global reputation 
    - You have a fingerprint for each Network
        - if you do high profile break-ins, fingerprint will grow
        - bigger fingerprint makes you easier to spot
        - like people have their eye out for you in particular
    - Networks have access control that determines who's allowed inside
        - Certain kinds of traffic may be allowed depending on role
    - Networks have security that can be broken if access control doesn't let you in
        - Upgrade to a different role
        - Difficulty depends on network properties, target role, and other attack vectors
    - Attack vectors allow you to target a network
        - e.g. open internet access, compromised network device, insider threat

HMW represent access control?
- list of available roles, global permissions
- e.g. "admin" role always has access to everything
"""
from enum import Enum


class AccessRole(Enum):
    ADMIN = 0
    MOD = 10
    USER = 20
    GUEST = 30


class AccessPermissions(Enum):
    FULLACCESS = 0
    ELEVATED = 10
    PARTIAL = 20
    READONLY = 30


class Network:
    def __init__(self):
        ### about you (as you relate to the network)
        self.local_rep = 0
        self.fingerprint = 0
        self.access_level = AccessRole.GUEST

        ### about the network itself, nothing to do with you
        self.access_control = {
            AccessRole.ADMIN: AccessPermissions.FULLACCESS,
            AccessRole.MOD: AccessPermissions.ELEVATED,
            AccessRole.USER: AccessPermissions.PARTIAL,
            AccessRole.GUEST: AccessPermissions.READONLY,
        }


if __name__ == "__main__":
    you = RogueAI()
