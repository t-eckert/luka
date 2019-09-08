# Luka 01

> This post is part of a series on the development of a reverse polish notation calculator web-app using Rust/Wasm, Vue, and TypeScript.

Date: 8 September 2019

## Branching Off

With the deployment pipeline set up from [Luka 00](./Luka00.md), any push to the `master` branch will kick off a new build/deploy instance and update the live website. That's great! But also I don't want to break it.

The solution here is to create a new `git` branch, in this case I'll call it `dev`. I can push code to this branch without setting off the pipeline. Taking this a step further, I will create a branch from `dev` called `add-rust-four-functions` and use that branch to create the first four functions for the calculator: +, -, *, /.

Then, when that work is done, I'll make a pull request to `dev`, then a pull request to `master`. I'll be reviewing my own code with each pull request, usually there is someone else who will approve the code review. Seems silly in certain ways and probably alien to some devs, but I like this model because it forces some level of reasonable caution around production.  

With this system in place, I am also going to bar myself from pushing to `master`. That way, the only code that gets into `master` (and thus into production) is code that has been merged in via pull request.  
