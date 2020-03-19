# Strelka

Strelka is an automated orbital launch controller for [Kerbal Space Program](https://www.kerbalspaceprogram.com/). It can launch rockets from Kerbal Space Centre to Low Kerbin Orbit, 100% computer controlled. 
Strelka is written in Rust, with [KRPC](https://krpc.github.io/krpc/) to communicate with KSP, and [Actix](https://github.com/actix/actix) actors. Rocket control is implemented as a collection of simple actors
running concurrently & responding to streams of data (altitude, apoapsis height, timer etc.) coming from KRPC. These actors can then issue commands to the Command actor which sends RPC commands (like perform
stage separation, set throttle etc.) to KRPC.

Strelka is named after the [Soviet space dog Strelka](https://en.wikipedia.org/wiki/Soviet_space_dogs#Belka_and_Strelka).

## Demo

[Watch a demo launch](https://youtu.be/VA8DUmSsBSg) from KSC to Low Kerbin Orbit, using a Vostok-like rocket.

[![Watch the Demo](https://i.imgur.com/GvhxO7m.png)](https://youtu.be/VA8DUmSsBSg)

## Try it

To start you'll need a copy of Kerbal Space Program (it's worth whatever they're asking for) with the KRPC mod installed.

Clone the code and `cargo build`. Build a rocket in KSP, put it on the launchpad then run `RUST_LOG=info cargo run`. Strelka will attempt to get it into a roughly circular
orbit above Kerbin. There's no guarantee Strelka can get every rocket you can fly into orbit, but that's just part of the fun (for now). Use `RUST_LOG=debug` or `RUST_LOG=trace`
to see more info about how Strelka is flying the rocket.

## What's next?

Strelka in its current form is a proof-of-concept and hopefully a starting point for some more interesting Kerbal automation projects.

Some ideas: 

 * Universal launch capability - Detect rocket features & calculate a way to get any orbit-capable rocket into space. 
 * Hoverslam algorithm for landing rockets a-la SpaceX Falcon 9 
 * There's some dodgy orbital mechanics maths in there that could do with a look-over by someone who got a maths qualification before their third attempt.
