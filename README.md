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
> My journey to be a Rustacean 🦀    
> <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="50" height="50"/>
> <img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" width="50" height="50"/>
> <img src="https://rustacean.net/assets/cuddlyferris.svg" width="50" height="50"/>

개발자로 꽤 오래 일했지만, 이상하게도 코딩하는 시간보다 다른 업무가 더 많았다.  
개발자 본연의 자세로 돌아가 가치 있는 소프트웨어를 만들고 싶었고, 사용해본 적이 없는 새로운 언어를 사용하고 싶었다.  
C 의 장점을 가지면서, 다양한 응용이 가능한 언어를 찾고 있는데, *Rust*가 그 Solution 인 듯하다.     
 
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

### Basic concepts

- Ownership
  -  the main purpose of ownership is to manage heap data 
  -  rules
    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
- reference
  - like a pointer in that it’s an address we can follow to access the data stored at that address
  - rules
    - 1 mutable reference or N immutable references.
    - always be valid.
- borrowing
  - the action of creating a reference
  - borrow checker
    - compares scopes to determine whether all borrows are valid
- enum
- match
- generics
  - abstract stand-ins for concrete types or other properties
    - concrete type (i32, f32, String ...)
    - generic types
      - Option<T>
      - Result<T, E>
      - Vec<T>
      - HashMap<K, V>
  - monomorphization
    - turning generic code into specific code by filling in the concrete types that are used when compiled
- trait
- lifetime specifier
- functional programming
- closure
- deref coercion
- smart pointer  
- Interior mutability
  - a mutable borrow to an immutable value

## Learning
### Step 1 : Rust fundamental (syntax & idioms)
- [x] The book
  - [x] The book community  번역 버젼  (https://rinthel.github.io/rust-lang-book-ko/)  
    - 챕터별로 번역 스타일이 다르고 가독성이 떨어짐 (22.05.19)   
  - [x] The book 한글판 : [러스트 프로그래밍 공식 가이드] (https://www.aladin.co.kr/shop/wproduct.aspx?ItemId=216867525)          
    - 전반적으로 번역이 좋다. 영어로 그대로 두어도 괜찮은 것들은 두면 좋겠다.
- [x] Easy Rust : Rust를 배웁시다. (Kor/Eng) by David MacLeod  
  - [Easy Rust Korean / 한국어판 강의](https://www.youtube.com/playlist?list=PLfllocyHVgsSJf1zO6k6o3SX2mbZjAqYE)  
    - 정렬된 강의 비디오 리스트 [TOC of Easy Rust](/toc_easyrust_playlist)
  - [Easy Rust Book](https://dhghomon.github.io/easy_rust/)       

### Step 2 : Coding Practice
muscle memory building(actually it doensnt have memory though)  
어느 정도 코딩할 수 있을 정도 지식이 생기면, 코딩 하면서 부족한 부분을 명확히 인지하게 되고 그 부분을 관련 자료 찾아 공부  

- [X] Rustlings : 조금 쉬운 편 (22.05.20)
[https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)
![Rustlings Completed](images/rustlings.png)

- [ ] **Exercism** : Online learning how to coding platform (코딩 인터뷰용이라기 보단, 교육적 목적 for free)  
[https://exercism.org/tracks/rust](https://exercism.org/tracks/rust)  
Solve coding exercises and get mentored to gain true fluency in your chosen programming languages.  
Web에서 문제를 풀어볼 수 있지만, WORK LOCALLY 방식으로 IDE의 도움받아서 코딩하는 것이 좋고,
Test Case를 통해 힌트를 얻고 test 작성법을 배울 수 있다.

  ![Exercsim Status](images/exercism.png)

- [ ] Coding Quiz(Leetcode 류의 문제) : 연습량 늘리기
  [/practice/training/](/practice/training/src)  

### Step 3 : Rust books

#### Step 3-1 : Zero 2 Production in RUST
- [ ] [Auth Web Microservice with rust using Actix-Web 4.0](https://gill.net.in/posts/auth-microservice-rust-actix-web1.0-diesel-complete-tutorial/)
- [ ] [Book : Zero To Production](https://www.zero2prod.com/index.html?country=Korea&discount_code=SEA60)  
* Actix 로 Back-end 만드는 과정, 최근에 Update 되었고, 책을 쓴 이유 중 하나가 새로 오는 팀원들을 위해 썼다고 한다.
  그래서, 초보용 책은 아니지만, 세심하게 배려했다고 서문과 리뷰에 언급되어 있어 안심하고 구매(했지만, 어렵다.)

<p float="left">
<!--   <img src="images/Untitled%202.png" width="600" /> <br> -->
  <img src="images/Untitled%203.png" width="600" /> 
</p>

### Step 4 : Learning by doing some projects
useful, big idea, start small
만들자 무언가 (생각나는 대로 적어 보고, 점차 구체화 할 것)
- Quant Stock Trading
  - Stock Trading SDK + Trader App (한국투자증권 KIS API)  
     https://github.com/goodsw4all/KIS_OpenAPI_in_Rust
- Bevy Engine App 
  - Metronome 
- 공공 Data API 를 활용해서 sth?
- Linux Kernel Module
- 기존의 C(or any others)로 된 것을 rewrite
- 번역
- 

## My own q/a

## References

![Untitled](images/Untitled%206.png)
