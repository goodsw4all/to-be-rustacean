# Learning Rust as a embedded C programmer  
  - [Intro](#intro)
    - [Good reasons for C Programmers](#good-reasons-for-c-programmers)
    - [Challenges](#challenges)
  - [Learning](#learning)
    - [Step 1 : Rust fundamental (syntax & idioms)](#step-1--rust-fundamental-syntax--idioms)
    - [Step 2 : Coding Practice](#step-2--coding-practice)
    - [Step 3 : Rust books written in English](#step-3--rust-books-written-in-english)
      - [Step 3-1 : Command-Line Rust](#step-3-1--command-line-rust)
      - [Step 3-2 : Zero 2 Production in RUST](#step-3-2--zero-2-production-in-rust)
    - [Step 4 : Learning by doing some projects](#step-4--learning-by-doing-some-projects)
  - [My own q/a](#my-own-qa)
  - [References](#references)
## Intro
> My journey to be a Rustacean π¦    
> <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="50" height="50"/>
> <img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" width="50" height="50"/>
> <img src="https://rustacean.net/assets/cuddlyferris.svg" width="50" height="50"/>

μ§κΈκ», μΌνλ©° μ΄μμλ μκ°λ€μ νκ³ ν΄ λ³΄λ κ½€ μ€λ λμ λλ¦ κ³ κ΅°λΆν¬ νμ§λ§, λ΄κ° μνλ μμ€μ κ°λ°μκ° λ κ²μ μλμλ€.  
λ΄κ° κ²½ννλ embedded system μμλ system building(bring-up) κ³Ό software κ°λ° μ΄ 5:5 μ λ μλ κ² κ°λ€.  
Embedded System Buildingμ κ½€ μ΅μν μμ€μ μ΄λ₯΄λ μΌλ Software κ°λ°μ λ§μ‘±ν  μμ€μ΄ μλλ€.  
λ΄κ° μ νκ³  μΆμλ κ²μ μννΈμ¨μ΄ κ°λ° κ·Έ μμ²΄ μλ κ² κ°κ³ , μμΌλ‘λ μννΈμ¨μ΄ κ°λ°μ λͺ°μνκ³  μΆλ€. 

μ΄μ μμ μ΅μ κ²λ§ λΆμ‘κ³  μμ κ²μ΄ μλλ―λ‘, μ¬μ©ν΄λ³Έμ μ΄ μλ μλ‘μ΄ μΈμ΄λ₯Ό μ¬μ©νκ³  μΆμλ€.  
C μ μ₯μ μ κ°μ§λ©΄μ, λ€μν μμ©μ΄ κ°λ₯ν μΈμ΄λ₯Ό μ°Ύκ³  μλλ°, *Rust*κ° κ·Έ Solution μΈ λ―νλ€   
μ΅μν κ²μμ λ²μ΄λλ κ²μ΄ λ€μ λλ ΅μ§λ§, μλν΄λ³΄λ κ²μ΄ ννμμ κ² κ°λ€.  
 
### Good reasons for C Programmers

- one-stop package & build system
- test built-in support
- memory safe coding
- compiler driven development
- dev community
- documentations
- modern programming concepts
- wide range from embedded to web
- easy TDD

### Challenges

- Ownership
- reference
- trait
- enum
- match
- Option / Result handling
- lifetime specifier
- functional programming
- smart pointer  
  Interior mutability
- closure

## Learning
### Step 1 : Rust fundamental (syntax & idioms)
- [x] The book νκΈ λ²μ­ λ²μ Ό  (https://rinthel.github.io/rust-lang-book-ko/)  
  - [x] μ½κΈ° : μ±ν°λ³λ‘ κ°λμ±μ΄ λ§μ΄ λ€λ₯΄λ€. μ΄ μ± μμ²΄κ° μ΄μ¬μλ₯Ό μν μ±μ μλκ³  μ΄λ €μ (22.05.19)   
  - [x] Cheat sheet
        https://github.com/donbright/rust-lang-cheat-sheet         
        
- [x] Easy Rust : Rustλ₯Ό λ°°μμλ€. (Kor/Eng) by David MacLeod  
νκ΅­λ§μ κ·Έλ₯ μ νλ μμ€μ΄ μλκΈ° λλ¬Έμ, μΈκ΅­μΈμ΄μ§λ§ νκ΅­μ΄λ‘ κ°μνλ κ²μ΄ μ ν λ¬Έμ  μλ€.  
[Easy Rust Korean / νκ΅­μ΄ν κ°μ](https://www.youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE)  
[Easy Rust Book](https://dhghomon.github.io/easy_rust/)  
  μ λ ¬λ κ°μ λΉλμ€ λ¦¬μ€νΈ [TOC of Easy Rust](/toc_easyrust_playlist)

### Step 2 : Coding Practice
muscle memory building(acturally it doensnt have memory though)  
μ΄λ μ λ μ½λ©ν  μ μμ μ λ μ§μμ΄ μκΈ°λ©΄, μ½λ© νλ©΄μ λΆμ‘±ν λΆλΆμ λͺνν μΈμ§νκ² λκ³  κ·Έ λΆλΆμ κ΄λ ¨ μλ£ μ°Ύμ κ³΅λΆ

- [ ] Coding Quiz(Leetcode λ₯μ λ¬Έμ ) : μ°μ΅λ λλ¦¬κΈ°
  [/practice/training/](/practice/training/src)
  

- [X] Rustlings : μ‘°κΈ μ¬μ΄ νΈ (22.05.20)
[https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
![Rustlings Completed](images/rustlings.png)

- [ ] **Exercism** : Online learning how to coding platform (μ½λ© μΈν°λ·°μ©μ΄λΌκΈ° λ³΄λ¨, κ΅μ‘μ  λͺ©μ  for free)  
[https://exercism.org/tracks/rust](https://exercism.org/tracks/rust)  
Solve coding exercises and get mentored to gain true fluency in your chosen programming languages.  
Webμμ λ¬Έμ λ₯Ό νμ΄λ³Ό μ μμ§λ§, WORK LOCALLY λ°©μμΌλ‘ IDEμ λμλ°μμ μ½λ©νλ κ²μ΄ μ’κ³ ,
Test Caseλ₯Ό ν΅ν΄ ννΈλ₯Ό μ»κ³  test μμ±λ²μ λ°°μΈ μ μλ€.

  ![Exercsim Status](images/exercism.png)

### Step 3 : Rust books written in English
   μμλ₯Ό end to endλ‘ μ½μΌλ©΄μ κ³΅λΆνλ κ²μ΄ μκ°κ³Ό λΈλ ₯μ΄ λ λ€κ² μ§λ§, μ°λ¦¬λ§ μλ£κ° λ³λ‘ μκΈ° λλ¬Έμ μ νμ μ¬μ§κ° μλ€.  
   μμ΄κ° νλ‘κ·Έλλ° μΈμ΄ μ΄μμΌλ‘ μ€μνλ€λ κ²μ λλλ‘ κΉλ¨Ήκ³  λ€μ κΉ¨λ«κ³  κ·Έλ°λ€.

#### Step 3-1 : Command-Line Rust
- [ ] https://www.oreilly.com/library/view/command-line-rust/9781098109424/

#### Step 3-2 : Zero 2 Production in RUST
- [ ] [Auth Web Microservice with rust using Actix-Web 4.0](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)
- [ ] [Book : Zero To Production](https://www.zero2prod.com/index.html?country=Korea&discount_code=SEA60)  
* backendμ κ΄μ¬μ΄ μκ²Όκ³  μ ν©ν μ±μ μ°Ύλ μ€ μ΄ μ±μ΄ Project-baseλ‘ λμμΈ λμ΄μμλ€. 
* μ΅κ·Όμ Update λμκ³ , μ±μ μ΄ μ΄μ  μ€ νλκ° μλ‘ μ€λ νμλ€μ μν΄ μΌλ€κ³  νλ€.
  κ·Έλμ, μ΄λ³΄μ© μ±μ μλμ§λ§, μΈμ¬νκ² λ°°λ €νλ€κ³  μλ¬Έκ³Ό λ¦¬λ·°μ μΈκΈλμ΄ μμ΄ μμ¬νκ³  κ΅¬λ§€(νμ§λ§, μ΄λ ΅λ€.)

<p float="left">
  <img src="images/Untitled%202.png" width="600" /> <br>
  <img src="images/Untitled%203.png" width="600" /> 
</p>

### Step 4 : Learning by doing some projects
useful, big idea, start small
λ§λ€μ λ¬΄μΈκ° (μκ°λλ λλ‘ μ μ΄ λ³΄κ³ , μ μ°¨ κ΅¬μ²΄ν ν  κ²)
- Linux Kernel Module
   - C -> Rust
- CLI app
   - Stock Trading SDK + Trader App (νκ΅­ν¬μμ¦κΆ KIS API)  
     https://github.com/goodsw4all/KIS_OpenAPI_in_Rust
- solana Dapp
- WASM
- κ³΅κ³΅ Data API λ₯Ό νμ©ν΄μ sth?

## My own q/a

## References

![Untitled](images/Untitled%206.png)
