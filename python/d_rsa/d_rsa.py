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

# *** ATTENTION ***
# To use d_rsa with other sources of random data, like /dev/urandom or OpenSSL,
# you must call it like:
#
# - /dev/urandom
#  $ hexdump -vn256 -e'"%08X"' /dev/urandom | d_rsa
#
# - OpenSSL:
#  $ openssl rand -hex 256 | d_rsa
#
# - random_generator:
#  $ random_generator password cf 100 32 256 | d_rsa
#
# This is necessary as d_rsa only accepts hex string as input!!

from Crypto.PublicKey import RSA
import gmpy2
from gmpy2 import mpz
  
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
    
    sk = RSA.construct((int(n), e, int(d)))
    pk = sk.publickey()
    
    with open('python.pem','wb') as pem:
        pem.write(sk.export_key('PEM'))
        pem.close()
    
    with open('python.pub','wb') as pem:
        pem.write(pk.export_key('PEM'))
        pem.close()
    