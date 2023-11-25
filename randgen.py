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
import time
import subprocess
import matplotlib.pyplot as plt

def speed():
    x1_rounds = [x for x in range(0,10000,100)]
    x2_confusionstring = string.ascii_letters[:5]
    
    # Testing for increasing number of rounds with confusion string of size 2
    y1_roundstime = []
    for x1 in x1_rounds:
        # Rust
        start = time.time_ns()
        subprocess.run(
            ['./rust/random_generator/target/release/random_generator',
             'password1',
             'cs',
             str(x1),
             '32',
             '256'],
            stdout = subprocess.DEVNULL
        )
        y1_roundstime.append(time.time_ns() - start)
        # TODO Haskell
    
    plt.subplot(1, 2, 1)
    plt.plot(x1_rounds, y1_roundstime, label="rust")
    plt.xlabel("Number of rounds")
    plt.ylabel("Time")
    plt.title("Round increase")
         
    # Testing for increasing size of confusion string and 100 rounds
    y2_confusionstringtime = []
    x2_confusionstringarray = []
    for x2 in range(1, len(x2_confusionstring)):
        # Rust
        start = time.time_ns()
        subprocess.run(
            ['./rust/random_generator/target/release/random_generator',
             'password1',
             x2_confusionstring[:x2],
             '100',
             '32',
             '256'],
            stdout = subprocess.DEVNULL
        )
        y2_confusionstringtime.append(time.time_ns() - start)
        x2_confusionstringarray.append(x2_confusionstring[:x2])
        # TODO Haskell
            
    plt.subplot(1, 2, 2)
    plt.title("Confusion string size increase")
    plt.xlabel("Confusion string size")
    plt.ylabel("Time")
    plt.plot(x2_confusionstringarray, y2_confusionstringtime, label="rust")
    
    plt.show()
    
        
    
def stdout():
    # Must call random generator to return 256 bytes (2048 bits)
    subprocess.run(
        ['./rust/random_generator/target/release/random_generator',
        'password1',
        'cs',
        '10',
        '32',
        '256'],
    )

#def graphs():

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    
    parser.add_argument(
        "-s",
        "--speed",
        action="store_true"
    )
    
    parser.add_argument(
        "-o",
        "--stdout",
        action="store_true"
    )
    
    args = parser.parse_args()
    
    if args.speed:
        speed()
    
    if args.stdout:
        stdout()