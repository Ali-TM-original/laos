

## Laos
<H3>
	A CAIE A-level assembly table solver 
</h3>

### Status ⇒ development
### Current Code Quality ⇒ bad 


## Example code
```js
STARTPROG
    VARIABLES
        VAR IX 0
        VAR ACC 0
        POSITION 365 1
        POSITION 366 3
        POSITION 367 65
        POSITION 368 66
        STARTLINE 200
    ENDVARIABLES
    # main code starts here
    LDD 365 # this is line 200 according to our STARTLINE variable and relative addressing
    CMP 366
    JPE 209
    INC ACC
    STO 365
    MOV IX
    LDX 365
    OUT
    JMP 200
    END
ENDPROG


```

## Basic Syntax and working

<image
	src="./showcase/Tokens.png"
/>

