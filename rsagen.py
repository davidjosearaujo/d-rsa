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

# Usage
#
# hexdump -vn256 -e'"%08X"' /dev/urandom | python3 rsagen.py -py 

import argparse
import subprocess
import os

if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        usage="""hexdump -vn256 -e'"%%08X"' /dev/urandom | python3 rsagen.py [OPTION]"""
    )
    
    parser.add_argument(
        "-rs",
        "--rust",
        action="store_true",
        help="Rust RSA key generator"
    )
    
    parser.add_argument(
        "-py",
        "--python",
        action="store_true",
        help="Python RSA key generator"
    )
    
    args = parser.parse_args()
    
    print("[+] Ensuring rust binaries are compiled...")
    os.system("cd rust/d_rsa && cargo build --release > /dev/null 2>&1")
    os.system("cd rust/random_generator && cargo build --release > /dev/null 2>&1")
    
    rand_stream = input()
    
    if args.rust:
        print("[+] Generating Rust RSA keys...")
        subprocess.run(
            ['./rust/d_rsa/target/release/d_rsa'],
            input=rand_stream.encode()
        )
        print("[!] Rust keys generated!")
        
    if args.python:
        print("[+] Generating Python RSA keys...")
        subprocess.run(
            ['python3',
            'python/d_rsa/d_rsa.py'],
            input=rand_stream.encode() 
        )
        print("[!] Python keys generated!")