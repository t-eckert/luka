# Luka 01

> This post is part of a series on the development of a reverse polish notation calculator web-app using Rust/Wasm, Vue, and TypeScript.

Date: 8 September 2019

## Branching Off

With the deployment pipeline set up from [Luka 00](./Luka00.md), any push to the `master` branch will kick off a new build/deploy instance and update the live website. That's great! But also I don't want to break it.

The solution here is to create a new `git` branch, in this case I'll call it `dev`. I can push code to this branch without setting off the pipeline. Taking this a step further, I will create a branch from `dev` called `add-rust-four-functions` and use that branch to create the first four functions for the calculator: +, -, *, /.

Then, when that work is done, I'll make a pull request to `dev`, then a pull request to `master`. I'll be reviewing my own code with each pull request, usually there is someone else who will approve the code review. Seems silly in certain ways and probably alien to some devs, but I like this model because it forces some level of reasonable caution around production.  

All work will follow this branching pattern:

![Branching pattern for this project. At the start, a single branch called `master` exists. Then, a branch from that is created called `dev`. From that branch, a new branch is created called `feature0`. On this branch several commits are made. That branch is merged back into `dev`. The `dev` branch is merged back into `master`.](./images/GitFlow.png)

> This is a good time to note that I steal a lot of the color pallets for quick images like the one above from [flatuicolors.com](https://flatuicolors.com/). They have done the hard work of finding which colors go together well.

## Rusty on Arithmetic

Now I can write some code! On the `add-rust-four-functions`, I created a library in Rust using `cargo new calculator --lib`. For now, I'm going to call this library `calculator` because it will do all the calculations for the project.

The model I'm thinking of using -- that I'm pretty sure will work is to have a `stack` object with methods that change the state of the `stack` and return its whole state. If I was writing the entire project in Rust, I wouldn't use this model, but I like the idea of just having this "functional" interface so the TypeScript only needs to directly query for the state of the `stack` once (when it's initialized).

If I find later on that this isn't the right model, I'll change it.  
