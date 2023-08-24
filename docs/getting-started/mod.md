# Getting Started

Welcome to the ***Getting Started*** section! This is the perfect place to begin your journey with our project. Here, we'll provide you with all the essential information and steps to get up and running quickly.

This project is a framework in the *Rust* programming language, designed so that integration with other languages is easy and seamless. The main focus was on building the core of the project in *Rust*, which ensures high reliability.

Our goal is to provide you with a flexible and powerful solution for processing data and performing checks.

Explore the documentation and create your own solutions with Flexible Inspect!


## Installation

All versions are uploaded to official registries where libraries for languages are stored. The project currently provides implementations for several languages:

=== "Rust"

    ``` sh
    cargo add flexible_inspect_rs
    ```

=== "JavaScript/TypeScript"

    ``` sh
    
    # If you need to validate directly in the browser or using webpack use
    npm install flexible_inspect_js_web
    # or
    npm install flexible_inspect_js_node
    ```
    !!! warning "Importing the library"
        The import differs depending on the version, see further in the documentation under `The first run`.
    

=== "Python"

    ``` sh
    pip install flexible_inspect_py
    ```
        

??? Failure "Package(version *x.y.z*) not found?"

    If your package manager currently gives the error : 
    
    > `that this package/version does not exist.` 
    
    Perhaps a new version has been released at the moment, (after a new version is published, it takes **20-25** minutes to build for all languages, try again after this time).
    
    Last build status : 
    
    [![CI/CD](https://github.com/m62624/flexible_inspect/actions/workflows/ci_cd.yml/badge.svg)](https://github.com/m62624/flexible_inspect/actions/workflows/ci_cd.yml)

{% include "./basic_concepts_and_terms.md" %}

## The first run
{% include "./first_run.md" %}