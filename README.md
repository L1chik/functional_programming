# Practical system programming
***

## -> _Arithmetic expression evaluator_
> **Analyzing the problem**

addition (+), subtraction (-), multiplication (*), division (/), power (^), the negative prefix (-), end of file ();
>**Building the tokenizer**
> 
Tokenizer reads characters from an arithmetic expression and translates it into token.
> 
>**Building the parser**
> 
Parser constructs tree of nodes with each node representing a token. Ast is a recursive tree structure of token nodes,
the root node is a token, which contains child that are also tokens. 

>**Building the evaluator**
> 
AST is contructed -> parses each node in the node tree recursively and arrives at the final value.
> 
>**Error handling**
> 
>**Building CLI**

***
## -> _Template engine_
>**Analyzing the problem**

A template engine can be used to generate dynamic HTML pages. It will contain parser and HTML generator.

***
## -> _Image scaling_
>**Analyzing the problem**

Resizes single/multiple images in a source folder to a specified size. Provides statistics from the image files.

* Supported formats: jpg, png
* Resize: size: [ small/medium/large | 200/400/800 ], mode: [ all/single ] 
* Stats: Amount of images in folder, total size of all images in this folder

**Usage:**
```
./imagecli resize [-s | --size] <small | medium | large>
                  [-m | --mode] <single | all>
                  --src <path-to-image-if-single | path-to-folder-if-all>
                  
./imagecli stats --src <path-to-folder>
```

***
## -> _RStat_

Shell command that computes source code metrics for Rust source files: Blanks, Comments, Line of codes.
```
./rstat <path to dir or file>
```
***
## -> _Text viewer_
Terminal text interface to load file from a directory and view the content.