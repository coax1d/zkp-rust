# Zero-Knowledge Implementations in Rust

This repository contains a personal library of projects that demonstrate various Zero-Knowledge (ZK) implementations using the Rust programming language. The aim is to showcase different ZK techniques and provide a comprehensive understanding of their usage, strengths, and limitations.

## Table of Contents

- [Introduction](#introduction)
- [Projects](#projects)
  - [ZK-SNARKs](#zk-snarks)
  - [ZK-STARKs](#zk-starks)
  - [Bulletproofs](#bulletproofs)
  - [Sonic](#sonic)
  - [Supersonic](#supersonic)
  - [Fractal](#fractal)
- [Dependencies](#dependencies)
- [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Introduction

Zero-Knowledge proofs allow a prover to convince a verifier that a statement is true, without revealing any information about the statement other than its validity. These cryptographic techniques have significant applications in privacy-preserving systems, such as cryptocurrencies and secure voting systems.

The Rust programming language offers strong safety guarantees, making it a fitting choice for implementing cryptography and Zero-Knowledge proofs. This repository showcases various ZK techniques in Rust, aiming to provide a starting point for those interested in learning more about this fascinating area.

## Projects

### ZK-SNARKs

ZK-SNARKs (Zero-Knowledge Succinct Non-Interactive Argument of Knowledge) are a family of Zero-Knowledge proofs that are succinct, meaning they can be quickly verified, and non-interactive, meaning they do not require back-and-forth communication between the prover and verifier. This project implements a basic ZK-SNARK using the Rust programming language.

### ZK-STARKs

ZK-STARKs (Zero-Knowledge Scalable Transparent ARguments of Knowledge) are a newer generation of Zero-Knowledge proofs that do not rely on a trusted setup and offer improved scalability. This project contains a Rust implementation of a simple ZK-STARK, demonstrating its key features and performance.

### Bulletproofs

Bulletproofs are short and efficient Zero-Knowledge proofs, particularly useful for range proofs and confidential transactions. This project demonstrates a Rust implementation of Bulletproofs.

### Sonic

Sonic is a Zero-Knowledge proof system that allows for a universal and updateable structured reference string, making it more practical for real-world applications. This project provides a Rust implementation of Sonic, highlighting its key features and benefits.

### Supersonic

Supersonic is a follow-up to Sonic that further improves performance and proof size. This project contains a Rust implementation of Supersonic, showcasing the advancements made in comparison to Sonic.

### Fractal

Fractal is a transparent, post-quantum secure, and succinct Zero-Knowledge proof system. This project demonstrates a Rust implementation of Fractal, highlighting its key features and its potential for future cryptographic systems.

## Dependencies

This project requires the following dependencies:

- Rust (latest stable version)
- Cargo (latest stable version)

Some projects may require additional dependencies. Please refer to their individual `Cargo.toml` files for more information.

## Installation

clone repository

# Partially Generated by Chat-GPT GPT-4
