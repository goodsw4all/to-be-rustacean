
  [Learning Rust as a embedded C programmer](#learning-rust-as-a-embedded-c-programmer)
  - [Intro](#intro)
    - [Good reasons for C Programmers](#good-reasons-for-c-programmers)
    - [Challenges](#challenges)
  - [Learning](#learning)
    - [Step 1 : Rust fundamental (systax & idioms)](#step-1--rust-fundamental-systax--idioms)
    - [Step 2 : Coding Practice](#step-2--coding-practice)
    - [Step 3 : 원서로 공부하기](#step-3--원서로-공부하기)
      - [Step 3-1 : Command Line Applications in Rust](#step-3-1--command-line-applications-in-rust)
      - [Step 3-2 : Zero 2 Production in RUST](#step-3-2--zero-2-production-in-rust)
    - [Step 4 : Learning by doing some projects](#step-4--learning-by-doing-some-projects)
  - [References](#references)

# Learning Rust as a embedded C programmer  

## Intro
> My journey to be a Rustacean   
> <img src="https://rustacean.net/assets/rustacean-flat-happy.svg" width="50" height="50"/>
> <img src="https://foundation.rust-lang.org/img/rust-logo-blk.svg" width="50" height="50"/>
> <img src="https://rustacean.net/assets/cuddlyferris.svg" width="50" height="50"/>

어느날 문득 고인물 개발자가 되었다는 것을 깨닫게 되고, 일하며 살아왔던 시간들을 회고해 보니     
꽤 오랜 동안 나름 고군분투 했지만, 내가 원하는 수준의 개발자가 된 것은 아니었다.  
embedded system 에서는 system building 과 firmware coding 이 7:3 정도 였던 것 같다.  
앞으로는 소프트웨어 개발에 몰입하고 싶다. 
 
unlearn 하고 learn 하는 과정이 쉽지 않겠지만, 실패하더라도 시도해 보는 수 밖에 없다.   

이왕 잘 하던것만 붙잡고 있을 것이 아니라면, 사용해본적이 없는 완전히 미지의 언어를 사용하고 싶었다.  
C 의 장점을 가지면서, 다양한 응용이 가능한 언어를 찾고 있는데, *Rust*가 그 Solution 인 듯하다   
익숙했던 device들과 씨름하는 개발을 넘어, 넓고 넓은 바깥 세계의 개발로 나가는 여정이라 험난 할 듯하다.  
 
### Good reasons for C Programmers

- one-stop package & build system

- test built-in support

- memory safe coding

- compiler driven development

- dev community

- documentations

- modern programming concepts

- wide range from embedded to web

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
### Step 1 : Rust fundamental (systax & idioms)
- The book 한글버젼 : https://rinthel.github.io/rust-lang-book-ko/
- Easy Rust : Rust를 배웁시다. (Kor/Eng) by David MacLeod  
한국말을 그냥 잘 하는 수준이 아니기 때문에, 외국인이지만 한국어로 강의하는 것이 전혀 문제 없다.
[https://dhghomon.github.io/easy_rust/](https://dhghomon.github.io/easy_rust/)
[Easy Rust Korean / 한국어판](https://www.notion.so/d0f614ab40194dfcac980bc6632d4eaf)

### Step 2 : Coding Practice

muscle memory building(acturally it doensnt have memory though)
어느 정도 코딩할 수 있을 정도 지식이 생기면, 코딩 하면서 부족한 부분을 명확히 인지하게 되고 그 부분을 관련 자료 찾아 공부

Rustlings : 조금 쉬운 편
[https://github.com/rust-lang/rustlings](https://github.com/rust-lang/rustlings)

**Exercism** : Online learning how to coding platform (코딩 인터뷰용이라기 보단, 교육적 목적 for free)  
[https://exercism.org/tracks/rust](https://exercism.org/tracks/rust)  
Solve coding exercises and get mentored to gain true fluency in your chosen programming languages.

WORK LOCALLY 로 VS Code 등 IDE를 도움을 받아 코딩하는 것이 좋고,
Test Case를 통해 힌트를 얻고 test 작성법을 배울 수 있다.

갈 길이 아직 많이 남았다

![Untitled](images/Untitled%201.png)

### Step 3 : 원서로 공부하기
   영어로 책을 end to end로 읽으면서 공부하는 것이 쉬운 것은 아니지만, 잘 할 수 있어야 하고, 아주 느리게 나아지고 있다.  
   프로그래머에게 영어는 프로그래밍 언어 만큼 중요하다.

#### Step 3-1 : Command Line Applications in Rust
https://rust-cli.github.io/book/index.html

#### Step 3-2 : Zero 2 Production in RUST
https://www.zero2prod.com/index.html?country=Korea&discount_code=SEA60
책 선택 이유
- backend에 관심이 생겼고 적합한 책을 찾던 중 이 책이 Project-base로 디자인 되어있었다. (몇 가지 project base 책들이 있음),
- 최근에 Update 되었고, 책을 쓴 이유 중 하나가 새로 오는 팀원들을 위해 썼다고 한다.
  그래서, 초보 베이스 책은 아니지만, 세심하게 배려했다고 서문과 리뷰에 언급되어 있어 안심하고 구매(했지만, 어렵다.)

![Untitled](images/Untitled%202.png)

![Untitled](images/Untitled%203.png)

좀 어렵게 느껴지지만, 이미 시작한 이상 끝은 봐야지.

![Untitled](images/Untitled%204.png)


### Step 4 : Learning by doing some projects
useful, big idea, start small
만들자 무언가 (생각나는 대로 적어 보고, 점차 구체화 할 것)
- Rpi4 Project
   - Linux Kernel Module
   - GPIO control
- CLI app
- solana Dapp
- WASM
- 공공 Data 를 활용해서 API server 를 만들기

## References

![Untitled](images/Untitled%205.png)

![Untitled](images/Untitled%206.png)
