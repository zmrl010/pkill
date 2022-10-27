# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v1.0.1 (2022-10-27)

<csr-id-6438a77ed086ff50be9551d06b7a892e6f969329/>
<csr-id-1b9a5d8883351205eee1c017ce0932db7f58ddfe/>
<csr-id-624803fdc5d798c33699aa8ed198266355ce7f43/>
<csr-id-16179a986942b89ba3885f4b4738ebb93b9ef742/>
<csr-id-ab4c22db70a31a498a05ed7558b354f60586c119/>
<csr-id-da5359c8d1619a9970eb80a22b423b7cc5a264d7/>
<csr-id-b8912cd5389edebc738d0f84c735f05cb74d18fa/>
<csr-id-7454cedf35ade1cc4df1f67c6d61a3229bc941c4/>

### Chore

 - <csr-id-6438a77ed086ff50be9551d06b7a892e6f969329/> cleanup
 - <csr-id-1b9a5d8883351205eee1c017ce0932db7f58ddfe/> descriptions for query struct

### Documentation

 - <csr-id-0d82df7e6f9e041e66ff36c7ef04d6ef1e9e02a7/> add readme

### New Features

 - <csr-id-1f2535260d53bb74c390347781b3bcaad7ec83e1/> automatically generate man pages
 - <csr-id-27f7b779b64301dc8bd64a3b49b9c78dac0e9b7e/> submit query target args
   arguments are now a vector of queries rather than singular parameters
 - <csr-id-b918cd03bc08e61b4945e550680ebf5252110662/> use trait params
 - <csr-id-6c40955f97c9dd2ff54f1ef58734636e4526ba78/> use anyhow instead of thiserror
   application is small enough where custom error types aren't needed
 - <csr-id-2c52d7acca41c2c675e51874359aede0c5cb1278/> kill processes by name
 - <csr-id-1c9b3fdb9030f89d4205c3ca3ee273d44762f9fb/> pull cli metadata from manifest
 - <csr-id-f12494eca18375e3be5a4cb5c55c2d9aeba506bf/> kill process by PID

### Bug Fixes

 - <csr-id-a4e604bc7921a0aa303431d2c4d89b912c5cf741/> require one of pid or name

### Refactor

 - <csr-id-624803fdc5d798c33699aa8ed198266355ce7f43/> split into workspace
 - <csr-id-16179a986942b89ba3885f4b4738ebb93b9ef742/> use map or else which is lazy
 - <csr-id-ab4c22db70a31a498a05ed7558b354f60586c119/> borrow sys from main fn
 - <csr-id-da5359c8d1619a9970eb80a22b423b7cc5a264d7/> process module
 - <csr-id-b8912cd5389edebc738d0f84c735f05cb74d18fa/> split result and error modules
 - <csr-id-7454cedf35ade1cc4df1f67c6d61a3229bc941c4/> extract process killing fn

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release over the course of 3 calendar days.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - split into workspace ([`624803f`](https://github.com/zmrl010/pkill/commit/624803fdc5d798c33699aa8ed198266355ce7f43))
    - cleanup ([`6438a77`](https://github.com/zmrl010/pkill/commit/6438a77ed086ff50be9551d06b7a892e6f969329))
    - use map or else which is lazy ([`16179a9`](https://github.com/zmrl010/pkill/commit/16179a986942b89ba3885f4b4738ebb93b9ef742))
    - descriptions for query struct ([`1b9a5d8`](https://github.com/zmrl010/pkill/commit/1b9a5d8883351205eee1c017ce0932db7f58ddfe))
    - automatically generate man pages ([`1f25352`](https://github.com/zmrl010/pkill/commit/1f2535260d53bb74c390347781b3bcaad7ec83e1))
    - submit query target args ([`27f7b77`](https://github.com/zmrl010/pkill/commit/27f7b779b64301dc8bd64a3b49b9c78dac0e9b7e))
    - borrow sys from main fn ([`ab4c22d`](https://github.com/zmrl010/pkill/commit/ab4c22db70a31a498a05ed7558b354f60586c119))
    - use trait params ([`b918cd0`](https://github.com/zmrl010/pkill/commit/b918cd03bc08e61b4945e550680ebf5252110662))
    - use anyhow instead of thiserror ([`6c40955`](https://github.com/zmrl010/pkill/commit/6c40955f97c9dd2ff54f1ef58734636e4526ba78))
    - process module ([`da5359c`](https://github.com/zmrl010/pkill/commit/da5359c8d1619a9970eb80a22b423b7cc5a264d7))
    - split result and error modules ([`b8912cd`](https://github.com/zmrl010/pkill/commit/b8912cd5389edebc738d0f84c735f05cb74d18fa))
    - require one of pid or name ([`a4e604b`](https://github.com/zmrl010/pkill/commit/a4e604bc7921a0aa303431d2c4d89b912c5cf741))
    - kill processes by name ([`2c52d7a`](https://github.com/zmrl010/pkill/commit/2c52d7acca41c2c675e51874359aede0c5cb1278))
    - extract process killing fn ([`7454ced`](https://github.com/zmrl010/pkill/commit/7454cedf35ade1cc4df1f67c6d61a3229bc941c4))
    - pull cli metadata from manifest ([`1c9b3fd`](https://github.com/zmrl010/pkill/commit/1c9b3fdb9030f89d4205c3ca3ee273d44762f9fb))
    - add readme ([`0d82df7`](https://github.com/zmrl010/pkill/commit/0d82df7e6f9e041e66ff36c7ef04d6ef1e9e02a7))
    - kill process by PID ([`f12494e`](https://github.com/zmrl010/pkill/commit/f12494eca18375e3be5a4cb5c55c2d9aeba506bf))
    - initial commit ([`f03a0b2`](https://github.com/zmrl010/pkill/commit/f03a0b26283054e4be07671aa4b79a314da5a889))
</details>

