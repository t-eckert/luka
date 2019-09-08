# Luka 00 - Design and Intent

Date: 7 September 2019

I have been thinking about this project for a few weeks - ever since I was on the train back from RustConf. I did a tutorial on WASM and Rust. [This One](https://rustwasm.github.io/docs/book/game-of-life/introduction.html). I was inspired to dig more into using WASM (wasm? WAsm?).

When I was learning React, I made a reverse polish notation calculator (RPN). You can see that project [here](http://rpn.herokuapp.com/). It's ok. This project is a new attempt at that project, but seeking to improve the following:  

- Greater mathematical precision: in the original, `(sqrt(2))^2 == 2.0000000000000004`, this is not true. I think that I can use the greater precision of Rust to get better mathematical precision. (I think...) The intent here is to use Rust to generate WASM that does the calculation and passes the result back to the JavaScript to render in Vue. The WASM code will generate functions that I can call from JavaScript.  
- Grown-up CSS: There's nothing wrong with Bootstrap. The original project used Bootstrap. Since working on that project I have learned CSS Grid and how to really style webpages. I want to do that for the purpose of practice, creating a more unique design, and shrinking the footprint of the site.  
- Learning!: I want to learn more about Rust, Vue, TypeScript, CSS, and WASM. That is the main driver.

As for these logs, the goal is to show how projects like this progress. It's not linear and I will make mistakes. I hope I can help others learn by making interesting mistakes. 

I want to start off by just having an endpoint for deployment. I'm going with Azure to be on brand. I know how to deploy a Vue app to Azure -- it's a static website. I don't know if that changes with the added WASM. That is one of the questions this project will answer.  

Before deployment, I will need at least something to deploy. Let's make a Vue app!

## Making a Vue App

I want to keep all the code together nicely, but that always shifts around in the early stages of a project. I am debating having a directory called `src` at the root or calling it `luka`. I'm not sure which is clearer. Is the project the whole of the repository, including the log, etc, or is it just what is in the directory.  

For now, I'll just call it `src`.  

In the newly created `src` directory, run `vue create .` (I'll admit I had to Google it). I created this project with Babel, TS, PWA, CSS Pre-processors, and the Linter.  

I have found TypeScript to save me so many times. I really enjoy using Vue and TS together.  

PWA support is great if people ever one day want to put this on their phones. Who knows.  

I am going to try and use class style syntax for this project. As for a CSS pre-processor, I'm using SCSS. If you are unfamiliar with SCSS, it compiles into CSS, but allows for a larger feature-set and can reduce the amount of styling you have to write manually. [Here](https://dzone.com/articles/introduction-of-scss) is a pretty good intro to SCSS.  

Once the app is generated, I can open it up in a browser with `npm run serve`.  

Great!

![The default web app created when vue create app is run. Text in the header reads "Welcome to Your Vue.js + TypeScript App".](./images/ImageOfDefaultGeneratedVueAppWithTypeScript.png)

Now that I look at it. I feel silly that there is a `src` directory within my `src` directory. Maybe I shouldn't I don't know. Things will probably move around before we're done.

## Azure Bound

I work on Azure. That's where I'm going to put this site. I'm just going to follow [this tutorial](https://passos.com.au/deploying-vue-js-to-azure-static-websites/) to set it up.

Instead of using Azure DevOps, I am using GitHub Actions. The YAML I wrote was a combination of the tutorial and using the [Azure/actions](https://github.com/Azure/actions) repo.

## Design

I created a Figma project [here](https://www.figma.com/file/rhgSHZhr0glvEwKqDv99rp/Luka?node-id=0%3A1). You should be able to comment on that. Let me know if you can't.  
