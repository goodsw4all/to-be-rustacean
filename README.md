# Learning Rust as a embedded C programmer  
## Intro
> C 에서 Rust 로 main 언어로 사용하기 위한 여정: My journey to be a Rustacean 🦀    
> <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="50" height="50"/>
> <img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" width="50" height="50"/>
> <img src="https://rustacean.net/assets/cuddlyferris.svg" width="50" height="50"/>

### Good reasons for C Programmers
- one-stop package & build system
- test built-in support, easy TDD
- memory safe coding
- compiler driven development
- dev community
- documentations
- modern programming concepts
- wide range from embedded to web

### Basic concepts
- statements and expressions
  - statements
    - serve mostly to contain and explicitly sequence expression evaluation : does not return, followed by `;`
      - declaration statements
      - expression statements
  - expressions
    - it always produces a value, and it may have effects : returns a value
    - evaluates to a value, and has effects during evaluation    
- ownership
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
  - give you a way of saying a value is one of a possible set of values
  - can have associated values
- pattern matching
  - patterns : Literals, Destructured arrays, enums, structs, or tuples, variables, wildcards, placeholders
  - matching : compare it to some value. If the pattern matches, use the value parts in our code
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
  - defines functionality a particular type has and can share with other types (shared behavier, like interface)
- lifetime
  - another kind of generic
  - ensure that references are valid as long as we need them to be
- functional programming
  - style
    - using functions as values by passing them in arguments,   
      returning them from other functions, assigning them to variables for later execution
  - closure
    - a function-like construct you can store in a variable  
  - iterator
    - a way of processing a series of elements
- smart pointers
  - pointer
    - general concept for a variable that contains an address in memory
  - smart pointer
    - data structures that act like a pointer but also have additional metadata and capabilities
  - deref coercion
    - converts a reference to a type that implements the Deref trait into a reference to another type
  - Interior mutability
    - a mutable borrow to an immutable value
  - common smart pointers
    - Box<T> for allocating values on the heap
    - Rc<T>, a reference counting type that enables multiple ownership
    - Ref<T>, RefMut<T>, accessed through RefCell<T>,    
      a type that enforces the borrowing rules at runtime instead of `compile time`

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

### Step 3 : Rust intermediate

#### Zero 2 Production in RUST
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
- GUI App 
  - Metronome 
- 공공 Data API 를 활용해서 sth?
- Linux Kernel Module
- 기존의 C(or any others)로 된 것을 rewrite
- 번역  
  - rust by example  
    https://goodsw4all.github.io/rust-by-example-ko/index.html
- 

## My own q/a

## References
### syntax
- Iterator   
  https://www.newline.co/@uint/rust-iterators-a-guide
- Smart pointers
![Untitled](images/Untitled%206.png)
