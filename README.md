# Luka

[![Build Status](https://dev.azure.com/teckert0973/res/_apis/build/status/t-eckert.luka?branchName=master)](https://dev.azure.com/teckert0973/res/_build/latest?definitionId=3&branchName=master)

## Backstory

I have been fascinated by reverse polish notation (RPN) calculators since college. Last year, I created an [RPN calculator web-app](http://rpn.herokuapp.com/) in React while I was learning the framework. It's not great. This is a new project with the same goals from scratch to push my learning of Rust, WASM, and Vue. I don't know if it will be successful, but my goal is to be radically transparent. I am going to log the whole project to share my progress.  

## Design

I am designing the UI in Figma. You can follow along and comment [here](https://www.figma.com/file/rhgSHZhr0glvEwKqDv99rp/Luka?node-id=0%3A1).

## Deployment

The application is compiled into a static website that is pushed to blob storage in Azure and served from there to [https://lukarpn.z14.web.core.windows.net](https://lukarpn.z14.web.core.windows.net).

The YAML that does this in the `azure-pipelines.yml` file.

## Development Environment

I am working in VS Code and vim depending on how I feel like writing. I like vim for long sessions in a single file, though I know there are good solutions for easy file switching. When I am going to be navigating around between different files, I like to use VS Code.  

I am starting the project on a 2015 MacBook Pro, but it is at the end of its rope so I may get a newer computer (XPS 15 🙏🏼) by the end of the project.  
