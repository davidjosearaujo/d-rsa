# Copyright 2023 David Araújo
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
#     http://www.apache.org/licenses/LICENSE-2.0
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

import argparse
import binascii
import hashlib

def create_confusion_pattern(confusion_string):
    confusion_pattern = []
    
    print(len(confusion_string.encode('utf-8')))
    
    #for chunk in confusion_string.encode('utf-8'):
        

def rand_byte_gen(password,confusion_string,rounds,seed_length,byte_amount):
    sha256 = bytes.fromhex(hashlib.sha256(confusion_string.encode('utf-8')).hexdigest())
    
    # Adjust the size of the seed array based on the desired seed length
    seed = hashlib.pbkdf2_hmac(
        'sha256',
        password.encode('utf-8'),
        sha256,
        rounds,
    )
    print(binascii.hexlify(seed))
    
    # Create confusion pattern base on confusion string
    confusion_pattern = sha256[:len(confusion_string)]
    
    # Compute a complex seed
    #complex_seed = complex_seed_generator(seed, confusion_pattern, rounds)
    
    print(binascii.hexlify(seed))
    
    return sha256
    

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        prog="Random Generator",
        description="Generates given amount of pseudo-random bytes",
    )
    
    parser.add_argument(
        "-pw",
        "--password",
        type=str,
        default="password",
    )
    
    parser.add_argument(
        "-cs",
        "--confusion_string",
        type=str,
        default="cf",
    )
    
    parser.add_argument(
        "-rd",
        "--rounds",
        type=int,
        default=100,
    )
    
    parser.add_argument(
        "-sl",
        "--seed_length",
        type=int,
        default=32,
    )
    
    parser.add_argument(
        "-ba",
        "--byte_amount",
        type=int,
        default=256,
    )
    
    args = parser.parse_args()
    
    buffer = rand_byte_gen(
        password=args.password,
        confusion_string=args.confusion_string,
        rounds=args.rounds,
        seed_length=args.seed_length,
        byte_amount=args.byte_amount
    )
    
    print(buffer)