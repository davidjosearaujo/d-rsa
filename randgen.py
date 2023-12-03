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
import string
import os
import time
import subprocess
import matplotlib.pyplot as plt

# COMPLETE
def speed():
    x1_rounds = [x for x in range(1,1000,100)]
    x1_confusionstring = string.ascii_letters[:4]
    
    # Testing for increasing number of rounds with confusion string of size 2
    y1_roundstime = []
    y2_roundstime = []
    print("[+] Round testing...")
    for x1 in x1_rounds:
        # Rust
        start = time.time_ns()
        subprocess.run(
            ['./rust/random_generator/target/release/random_generator',
             'password1',
             'cf',
             str(x1),
             '32',
             '256'],
            stdout = subprocess.DEVNULL
        )
        y1_roundstime.append(time.time_ns() - start)
        
        # Python
        start = time.time_ns()
        rounds_py = '--rounds=' + str(x1)
        subprocess.run(
            ['python3',
             'python/random_generator/random_generator.py',
             rounds_py
             ],
            stdout = subprocess.DEVNULL
        )
        y2_roundstime.append(time.time_ns() - start)
        
    
    plt.subplot(1, 2, 1)
    plt.plot(x1_rounds, y1_roundstime, label="Rust")
    plt.plot(x1_rounds, y2_roundstime, label="Python")
    plt.xlabel("Number of rounds")
    plt.ylabel("Time")
    plt.title("Round increase effect")
    plt.legend()
    
    print("[!] Round testing complete!")
    print("[+] Confusion string size testing...")
         
    # Testing for increasing size of confusion string and 100 rounds
    y1_confusionstringtime = []
    x1_confusionstringarray = []
    
    y2_confusionstringtime = []
    for x2 in range(1, len(x1_confusionstring)):
        # Rust
        start = time.time_ns()
        subprocess.run(
            ['./rust/random_generator/target/release/random_generator',
             'password2',
             x1_confusionstring[:x2],
             '100',
             '32',
             '256'],
            stdout = subprocess.DEVNULL
        )
        y1_confusionstringtime.append(time.time_ns() - start)
        x1_confusionstringarray.append(x1_confusionstring[:x2])
        
        # Python
        start = time.time_ns()
        cf_arg = '--confusion_string='+str(x1_confusionstring[:x2])
        subprocess.run(
            ['python3',
             'python/random_generator/random_generator.py',
             cf_arg
             ],
            stdout = subprocess.DEVNULL
        )
        y2_confusionstringtime.append(time.time_ns() - start)
            
    plt.subplot(1, 2, 2)
    plt.title("Confusion string size effect")
    plt.xlabel("Confusion string")
    plt.ylabel("Time")
    plt.yscale("log")
    plt.plot(x1_confusionstringarray, y1_confusionstringtime, label="Rust")
    plt.plot(x1_confusionstringarray, y2_confusionstringtime, label="Python")
    plt.legend()
    
    print("[!] Confusion string size testing complete!")
    
    plt.show()
    
        
# COMPLETE
def stdout():
    # Must call random generator to return 256 bytes (2048 bits)
    
    # Rust
    print("Rust stdout: ")
    subprocess.run(
        ['./rust/random_generator/target/release/random_generator',
        'password',
        'cf',
        '100',
        '32',
        '256'],
    )
    

    # Python
    print("\n\nPython stdout: ")
    subprocess.run(
        ['python3',
         'python/random_generator/random_generator.py',
         '-pw=password',
         '-cs=cf',
         '-rd=100',
         '-sl=32',
         '-ba=256'],
    )

# COMPLETE
if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        usage="randgen.py [OPTION]"
    )
    
    parser.add_argument(
        "-s",
        "--speed",
        action="store_true",
        help="generate Python vs. Rust speed test graphs"
    )
    
    parser.add_argument(
        "-o",
        "--stdout",
        action="store_true",
        help="print to stdout the random stream generated"
    )
    
    args = parser.parse_args()
    
    print("[+] Ensuring that Rust binaries are compiled...")
    os.system("cd rust/d_rsa && cargo build --release > /dev/null 2>&1")
    os.system("cd rust/random_generator && cargo build --release > /dev/null 2>&1")
    print("[!] Rust binaries compiled!")
    
    if args.speed:
        speed()
    
    if args.stdout:
        stdout()