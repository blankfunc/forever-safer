<h1 align="center">
	Forever Safer
</h1>

<p align="center">
	This project aims to assist every "Ruster" in facing the troubles of <strong>multi-threaded issues</strong>.
</p>

## Features
> This is a collection of projects.

+ Avoid all potential errors and ensure absolute security with minimal `Result<O, E>`.
+ Without `Tokio`, but also compatible with `Tokio`.
+ Ensure the security of any event in multi-threaded environments.
+ Ensure all are `sync` and eliminate `async`.

## Child Projects
### SegQueue
> with feature `seg-queue`.

Improvements to `crossbeam::queue::SegQueue`, supporting `peek` and `contains`.

Performance Test Results

### AtomicPoll
> with feature `atomic-poll`.

The ID auto increment function that can recycle and reuse IDs.

### InstantBus
> with feature `instant-bus`.

A broadcast distributor that *sends unilaterally*, *is received by multiple parties*, *discards without receiving*, and *has no historical messages*.