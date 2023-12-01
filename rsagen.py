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
import subprocess

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    
    parser.add_argument(
        "-rs",
        "--rust",
        action="store_true"
    )
    
    parser.add_argument(
        "-py",
        "--python",
        action="store_true"
    )
    
    args = parser.parse_args()
    
    rand_stream = input()
    
    if args.rust:
        subprocess.run(
            ['./rust/d_rsa/target/release/d_rsa'],
            input=rand_stream.encode()
        )
        
    if args.python:
        subprocess.run(
            ['python3',
            'python/d_rsa/d_rsa.py'],
            input=rand_stream.encode() 
        )