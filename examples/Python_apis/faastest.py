from cbnb import cfaas
from cbnb import ckvstore

import random as r

USER_ID = '123dfasdfasd4567890'
#FAAS_ID = 'e606a8bd-126e-4e6a-b2e7-0dcf758ce608'

#./cbnbcli  -c cbnb.com:7779 --userid 12312 storage

#./cbnbcli -c cbnb.com:7779 --userid safdsfad123  faas create -d fibexample -l 'Rust' -p 'fib(num2:u128) -> u128'
#./cbnbcli  -c cbnb.com:7779 --userid safdsfad123 faas publish -i 8354c56e-03e1-454c-8013-8eadef087d08
"""
fib = cfaas(USER_ID,FAAS_ID)
for i in range(0,10):
    print(fib.call(i))
"""

kv = ckvstore(USER_ID)

for i in 'dsfvbhjtyhr34567ikjhnb':
    print(kv.set({'username'+i:i*r.randint(100,1000)}))
    print(kv.get('username'+i))
