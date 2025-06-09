import requests
import time
import threading

from web3 import Web3, HTTPProvider
from dotenv import load_dotenv
import os

import json

load_dotenv()

node_url = os.getenv("RPC_URL")

w3 = Web3(HTTPProvider(node_url))

if w3.is_connected():
    print("Connected to Ethereum node!")
else:
    print("Failed to connect to Ethereum node.")
    exit()


def get_block_hash(block_number):
    # block_number = 22637843
    try:
        # Get the block object by number. The second parameter (True/False)
        # determines if full transaction objects are returned, or just hashes.
        # We only need the block data, so False is sufficient.
        block = w3.eth.get_block(block_number, False)

        if block:
            block_hash = block.hash.hex() # Convert bytes to hexadecimal string
            print(f"Block Number: {block_number}")
            print(f"Block Hash: {block_hash}")
            return block_hash
        else:
            print(f"Block {block_number} not found.")

    except Exception as e:
        print(f"An error occurred: {e}")
    
    
def call_endpoint_threaded():
    
    """Function to call your endpoint, suitable for a thread."""
    endpoint_url = f"https://eu-ssb.internal.tokkalabs.com/ethpath-mainnet/v1/optimal?sellToken=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48&buyToken=0x5aFE3855358E112B5647B952709E6165e1c1eEEe&sellAmount=3108444914&taker=0x9008d19f58aabd9ed0d60971565aa8510560ab41&tx=true&slippage=0&app=test_suite" # Replace
    print(f"[{time.ctime()}] [ThreadA] Calling endpoint: {endpoint_url}")
    try:
        response = requests.get(endpoint_url, timeout=10)
        response.raise_for_status()
        path_finder_response = response.json()
        print(f"  [ThreadA] Success calling path finder! Status Code: {response.status_code}")
        block_number = path_finder_response["blockNumber"]
        block_hash = get_block_hash(block_number)
        
        output_filename = f"./path_finder_output/path_finder_response_{block_number}.json"
        try:
            with open(output_filename, 'w') as f: # Use 'w' to overwrite, 'a' to append
                json.dump(path_finder_response, f, indent=4) # indent for pretty printing
            print(f" [ThreadA] Response saved to {output_filename}")
        except IOError as e:
            print(f" [ThreadA] Error writing response to file {output_filename}: {e}")
        
        try:
            response =  requests.post(
                url = "http://127.0.0.1:8080/state",
                json = 
                    {
                        "block_hash": block_hash,
                        "block_number": block_number
                    }
            )
            response.raise_for_status()
        except requests.exceptions.RequestException as e:
            print(f"  [ThreadB] Error calling test_server endpoint: {e}")
    except requests.exceptions.RequestException as e:
        print(f"  [ThreadA] Error calling endpoint: {e}")
    finally:
        # Schedule the next call after this one completes
        schedule_next_call()

def schedule_next_call():
    interval_seconds = 2 * 55
    # Create a Timer that will run call_endpoint_threaded after interval_seconds
    # daemon=True means the thread will exit automatically when the main program exits
    timer = threading.Timer(interval_seconds, call_endpoint_threaded)
    timer.daemon = True # Allows the main program to exit even if this thread is still running
    timer.start()
    print(f"[{time.ctime()}] [Main] Scheduled next call in {interval_seconds} seconds.")


if __name__ == "__main__":
    print("Starting threaded endpoint calls...")
    schedule_next_call() # Start the first call and schedule subsequent ones

    # Keep the main thread alive, otherwise daemon threads will exit immediately.
    # In a real application, your main thread would be doing other work (e.g., a web server).
    try:
        while True:
            time.sleep(1) # Sleep briefly to avoid busy-waiting
            # You can put other main thread logic here if needed
            # print("Main thread alive...") # Uncomment to see main thread running
            
            
    except KeyboardInterrupt:
        print("\nMain thread interrupted. Exiting.")