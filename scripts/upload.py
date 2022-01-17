import json
from redis import Redis
from splitstream import splitfile

redis = Redis()

redis.delete('events')

with open('data/db.stream', 'r') as infile:
    for event_json in splitfile(infile, format="json"):
        event = json.loads(event_json)
        redis.xadd('events', event)
