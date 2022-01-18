import pprint
import json
from redis import Redis
from splitstream import splitfile


redis = Redis()

profiles = {}


def ProfileRegistered(event):
    user_id = event[b'user_id'].decode()
    discord_username = event[b'discord_username'].decode()

    profiles[user_id] = {
        'user_id': user_id,
        'discord_username': discord_username,
        'credit': 1000,
        'jailed': False
    }


def ComradeHonored(event):
    to_user_id = event[b'to_user_id'].decode()
    by_user_id = event[b'by_user_id'].decode()
    amount = int(event[b'amount'].decode())
    reason = event[b'reason'].decode()

    profiles[to_user_id]['credit'] += amount


def ComradeJailed(event):
    to_user_id = event[b'to_user_id'].decode()
    by_user_id = event[b'by_user_id'].decode()
    reason = event[b'reason'].decode()

    profiles[to_user_id]['jailed'] = True


def ComradeUnjailed(event):
    to_user_id = event[b'to_user_id'].decode()
    by_user_id = event[b'by_user_id'].decode()

    profiles[to_user_id]['jailed'] = False


def ComradeDishonored(event):
    to_user_id = event[b'to_user_id'].decode()
    by_user_id = event[b'by_user_id'].decode()
    amount = int(event[b'amount'].decode())
    reason = event[b'reason'].decode()

    profiles[to_user_id]['credit'] -= amount
    print("reason:", reason)


for id, event in redis.xrange('events'):
    handler = {
        b'ProfileRegistered': ProfileRegistered,
        b'ComradeHonored': ComradeHonored,
        b'ComradeDishonored': ComradeDishonored,
        b'ComradeJailed': ComradeJailed,
        b'ComradeUnjailed': ComradeUnjailed,
    }[event[b'type']]

    handler(event)


print(json.dumps(profiles, indent=4))
