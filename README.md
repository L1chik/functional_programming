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

> Resizes multiple images in a source folder to a specified size. Provides statistics from the image files. 
>