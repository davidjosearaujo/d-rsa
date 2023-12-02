<!--
 Copyright 2023 David Araújo
 
 Licensed under the Apache License, Version 2.0 (the "License");
 you may not use this file except in compliance with the License.
 You may obtain a copy of the License at
 
     http://www.apache.org/licenses/LICENSE-2.0
 
 Unless required by applicable law or agreed to in writing, software
 distributed under the License is distributed on an "AS IS" BASIS,
 WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 See the License for the specific language governing permissions and
 limitations under the License.
-->

# Deterministic RSA key generation (D-RSA)

## Technologies used

For this projects, the two languages selected where **Rust** and **Python**.

Rust is an ideal choice for processing large numbers in RSA key generation due to its focus on **performance and safety**. Rust strikes a balance between high-performance requirements and robustness, making it well-suited for implementing the intricate calculations involved in RSA key generation.

Python's simplicity is key. Its clean syntax and high-level abstractions simplify complex calculations, facilitating quick and readable implementation. While not the fastest, Python's straightforward approach makes it an accessible and efficient choice for cryptographic tasks, emphasizing simplicity and ease of development.

## Pseudo-random generator

For this explanation we will focus on the Rust implementation as it is more complex to understand form the source code. Keep in mind that the Python implementation is follows as closely as possible the same 
functions, variable names, and overall structure, so the understanding of the Rust implementation is translatable to Python.



## RSA key pair generator

## rsagen and randgen

## Difficulties in implementation in seconds language