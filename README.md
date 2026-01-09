# Willow Inference Server - Rust Revamp

## Rationale

Ever Willow's initial public release in 2023, people have been asking if they could run Willow Inference Server without an NVIDIA GPU. Unfortunately, back then, there weren't any options that offered similar performance. If one has to wait multiple seconds for a voice command to be executed, one might as well grab their phone from their pocket, and do the action with their phone. For that reason, we decided to focus only on CUDA.

Fast forward to 2026, the "AI" landscape has drastically evolved. There are models out there that are almost as fast on CPU as Whisper on CUDA back in 2023. And with the passing of Willow's founder, the public, best-effort WIS server has disappeared, and the need for a WIS server that can run without expensive, power-hungry NVIDIA GPU has only increased.

This project is an attempt to provide a WIS that is fast enough without requiring an expensive NVIDIA GPU. It is in very early stages of development, far from feature-complete with the original Python WIS, but it handles STT fast enough to be usable. And as people have expressed interest in it, we're publishing it in its current form, so people can experiment with it.

