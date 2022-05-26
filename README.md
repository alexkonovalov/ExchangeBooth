# Solana Bootcamp Exchange Booth Project

## Table of Contents

-   [About](#about)
-   [Disclaimer](#disclaimer)
-   [Getting Started](#getting_started)
-   [Prerequisites](#prerequisites)

## About <a name = "about"></a>

Smart contract project that faciliates on-chain token exchanges. This is an implementation of [Day 3 Solana bootcamp project exercise](https://www.youtube.com/playlist?list=PLilwLeBwGuK7Z2dXft_pmLZ675fuPgkA0).

Client can perform several instructions on Exchange Booth program:

-   Initialize `npm run -- --ix=0`
-   Deposit `npm run -- --ix=1`
-   Close `npm run -- --ix=2`
-   Exchange `npm run -- --ix=3`
-   Withdraw `npm run -- --ix=4`

Since this progam is made primarily for demo purposes values for each instruction are hardcoded on a client.

Full specification:
[Exchange_Booth_Program_Spec.pdf](/Exchange_Booth_Program_Spec.pdf)

## Disclaimer <a name = "disclaimer"></a>

This is not production-ready smart contract. The goal of this project is to study and demonstrate the basic knowledge of Solana smart-contracts.

## Getting Started <a name = "getting_started"></a>

1. Make sure in local solana configuration `json_rpc_url` value is local test validator address or solana devnet address.
2. Run `npm run booth:cycle` to build and deploy _bpf_, prepare environment and run the sequense of all commands

## Prerequisites <a name = "prerequisites"></a>

-   [Node.js](https://nodejs.org/en/)
-   [Solana Tools](https://docs.solana.com/cli/install-solana-cli-tools)
