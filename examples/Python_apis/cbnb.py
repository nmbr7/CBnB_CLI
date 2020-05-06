import requests
import json



class cfaas(object):
    def __init__(self,uuid,faasid):
        self.uuid = uuid
        self.faasid = faasid

    def call(self,*params):
        url = 'http://faas.cbnb.com:7779/invoke'
        payload = {'faas_uuid': self.faasid, 'uuid':self.uuid,'params': [str(i) for i in params]}
        headers = {'content-type': 'application/json'}
        r = requests.post(url, data=json.dumps(payload), headers=headers)
        # currently returns the \n and other escape characters along with the response
        return r.text.strip()

class ckvstore(object):
    def __init__(self,uuid):
        self.uuid = uuid

    def get(self,key):
        url = 'http://kv.cbnb.com:7779/'+key
        payload = {'id':self.uuid}
        headers = {'content-type': 'application/json'}
        r = requests.post(url, data=json.dumps(payload), headers=headers)
        # currently returns the \n and other escape characters along with the response
        return r.text

    def set(self,kv):
        url = 'http://kv.cbnb.com:7779'
        payload = {'kv': {list(kv.keys())[0] : list(kv.values())[0]}, 'id':self.uuid}
        headers = {'content-type': 'application/json'}
        r = requests.put(url, data=json.dumps(payload), headers=headers)
        # currently returns the \n and other escape characters along with the response
        return r.text
