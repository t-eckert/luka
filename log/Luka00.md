# Luka 00 - Design and Intent

Date: 7 September 2019

I have been thinking about this project for a few weeks - ever since I was on the train back from RustConf. I did a tutorial on WASM and Rust. [This One](https://rustwasm.github.io/docs/book/game-of-life/introduction.html). I was inspired to dig more into using WASM (wasm? WAsm?).

When I was learning React, I made a reverse polish notation calculator (RPN). You can see that project [here](http://rpn.herokuapp.com/). It's ok. This project is a new attempt at that project, but seeking to improve the following:  

- Greater mathematical precision: in the original, `(sqrt(2))^2 == 2.0000000000000004`, this is not true. I think that I can use the greater precision of Rust to get better mathematical precision. (I think...) The intent here is to use Rust to generate WASM that does the calculation and passes the result back to the JavaScript to render in Vue. The WASM code will generate functions that I can call from JavaScript.  
- Grown-up CSS: There's nothing wrong with Bootstrap. The original project used Bootstrap. Since working on that project I have learned CSS Grid and how to really style webpages. I want to do that for the purpose of practice, creating a more unique design, and shrinking the footprint of the site.  
- Learning!: I want to learn more about Rust, Vue, TypeScript, CSS, and WASM. That is the main driver.

As for these logs, the goal is to show how projects like this progress. It's not linear and I will make mistakes. I hope I can help others learn by making interesting mistakes.

## Design

I created a Figma project [here](https://www.figma.com/file/rhgSHZhr0glvEwKqDv99rp/Luka?node-id=0%3A1). You should be able to comment on that. Let me know if you can't.  

## Deployment

I want to start off by just having an endpoint for deployment. I'm going with Azure to be on brand. I know how to deploy a Vue app to Azure -- it's a static website. I don't know if that changes with the added WASM. That is one of the questions this project will answer.  

Before deployment, I will need at least something to deploy.
