import requests
import json
import time

# Base URL for the API
base_url = 'http://127.0.0.1:8000'

# Function to add a transaction
def add_transaction(sender, receiver, amount):
    url = f'{base_url}/transaction'
    payload = {
        'sender': sender,
        'receiver': receiver,
        'amount': amount
    }
    headers = {'Content-Type': 'application/json'}
    response = requests.post(url, data=json.dumps(payload), headers=headers)
    return response.json()

# Function to get the last block
def get_last_block():
    url = f'{base_url}/last_block'
    response = requests.get(url)
    return response.json()

# Function to add a block
def add_block():
    url = f'{base_url}/add_block'
    response = requests.post(url)
    return response.json()

# Measure time taken for each API call
start_time = time.time()
print('Adding transactions...')
for i in range(10):
    print(add_transaction(f'sender_{i}', f'receiver_{i}', i * 10))

print('Time taken to add 10 transactions: %s seconds' % (time.time() - start_time))

start_time = time.time()
print('Getting last block...')
print(get_last_block())
print('Time taken to get last block: %s seconds' % (time.time() - start_time))

start_time = time.time()
print('Adding a block...')
print(add_block())
print('Time taken to add a block: %s seconds' % (time.time() - start_time))