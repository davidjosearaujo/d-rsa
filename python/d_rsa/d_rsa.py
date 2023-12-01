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

import base64
import gmpy2
from gmpy2 import mpz

class KeyPair:
    def __init__(self, pk, sk):
        self.pk = pk
        self.sk = sk
        
    def print(self):
        pk_file = open("py_rsa_pk.pub", "w")
        sk_file = open("py_rsa_sk.pem", "w")
        
        # Encoding factors of each key to Base64 stream
        pk_stream = self.pk.encode()
        sk_stream = self.sk.encode()
        
        # Writing public key file
        pk_file.write("---------- BEGIN RSA PUBLIC KEY ----------\n")
        for i in range(0, len(pk_stream), 64):
            pk_file.write(pk_stream[i: i+64])
            pk_file.write("\n")
        pk_file.write("---------- END RSA PUBLIC KEY ----------")
        pk_file.close()
        
        # Writing secret key file
        sk_file.write("---------- BEGIN RSA PRIVATE KEY ----------\n")
        for i in range(0, len(sk_stream), 64):
            sk_file.write(sk_stream[i: i+64])
            sk_file.write("\n")
        sk_file.write("---------- END RSA PRIVATE KEY ----------")
        sk_file.close()
                
class PublicKey:
    def __init__(self, n, e):
        self.n = n
        self.e = e
    
    def encode(self):
        return base64.b64encode(self.n).decode("utf-8") + base64.b64encode(self.e).decode("utf-8")

class SecretKey:
    def __init__(self, n, d):
        self.n = n
        self.d = d
    
    def encode(self):
        return base64.b64encode(self.n).decode("utf-8") + base64.b64encode(self.d).decode("utf-8")
    
def is_prime(num_str):
    num = mpz(num_str)
    return num.is_prime()

def turn_prime(number):
    # Turn prime
    #  1. LSB to 1
    #  2. Add 2 until it passes primality tests

    # Convert bytes to a bytearray to make it mutable
    number = bytearray(number)

    # Bitwise OR turns the LSB to 1
    number[-1] |= 0b00000001

    # Turn list into a string and then into a Big Unsigned Number
    big_number = mpz(int.from_bytes(number, byteorder='big'))

    # Increase the number until a prime number is found
    while not is_prime(str(big_number)):
        big_number += 2

    return big_number

if __name__ == "__main__":
    received_stream = input()
    
    # Split hex string into array of bytes and split it for p and q variables
    p = bytes.fromhex(received_stream[:int(len(received_stream)/2)])
    q = bytes.fromhex(received_stream[int(len(received_stream)/2):])
    
    # Turn p and q into prime numbers
    big_prime_p = turn_prime(p)
    big_prime_q = turn_prime(q)
    
    # Calculate modulus
    n = big_prime_p * big_prime_q
    
    # Calculate e
    e = 2 ** 16 + 1
    
    # Carmichael's totient function
    lambda_n = gmpy2.lcm(big_prime_p-1,big_prime_q-1)
    
    # Inverse modulus of ƛ(n)
    d = pow(e, -1, lambda_n)
    
    pk = PublicKey(gmpy2.to_binary(n), str(e).encode("utf-8"))
    sk = SecretKey(gmpy2.to_binary(n), gmpy2.to_binary(d))
    kp = KeyPair(pk, sk)
    
    # TODO: Getting bytes from the n do not match Rust version, although, n does. makes no sense...
    
    kp.print()