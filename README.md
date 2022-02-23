# Practical system programming
***

### Chapter 2 -> _Arithmetic expression evaluator_
> **Analyzing the problem**

addition (+), subtraction (-), multiplication (*), division (/), power (^), the negative prefix (-), end of file ();
>**Building the tokenizer**
> 
Tokenizer reads characters from an arithmetic expression and translates it into token.
> 
>* Building the parser
> 
Parser constructs tree of nodes with each node representing a token. Ast is a recursive tree structure of token nodes,
the root node is a token, which contains child that are also tokens. 
>* Building the evaluator
>* Error handling
>* Building CLI

---