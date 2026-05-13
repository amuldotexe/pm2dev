# Java Spring Boot Meta-Pattern Study Plan

Source roadmap: `/Users/amuldotexe/Downloads/Java_Spring_Boot_Roadmap.pdf`

Goal: learn Java Spring Boot backend engineering by constantly translating each concept into three other languages:

- ELI5 intuition: what it feels like before jargon.
- CS x math meta-pattern: the deeper shape underneath the tool.
- Rust and JavaScript/TypeScript comparison: how the same idea mutates across ecosystems.

This is an intensive plan: 40-50 hours per week for 12 weeks.

## Premise Check

The roadmap is a Java/Spring Boot backend path, but the deeper target is not just "learn Spring." The deeper target is to become fluent in backend ideas that recur across languages: types, state, graphs, resources, contracts, invariants, effects, boundaries, and feedback loops.

The useful premise is:

> Java/Spring teaches enterprise backend architecture through managed objects, annotations, dependency injection, relational persistence, and production infrastructure.

The meta-pattern premise is:

> Most backend concepts are disguised math or CS structures: sets of values, mappings, state machines, graphs, relations, protocols, constraints, proofs, and observations.

The comparison premise is:

> Rust usually makes constraints explicit in types, ownership, lifetimes, traits, and compile-time checks. Java/Spring often moves structure into classes, annotations, runtime containers, and conventions. JavaScript/TypeScript often splits the difference: dynamic runtime plus optional static modeling through TypeScript and framework conventions.

Use this study plan as a translation engine. Every week, ask:

1. What is the beginner picture?
2. What is the formal pattern?
3. How does Java/Spring encode it?
4. How would Rust force me to model it?
5. How would JS/TS make it flexible, dangerous, or ergonomic?
6. What project proves I understand it?

## Expert Lenses

| Lens | What it protects |
| --- | --- |
| Spring backend engineer | Practical Java/Spring correctness, maintainability, and production readiness. |
| CS x math teacher | Turns tools into durable concepts: sets, functions, graphs, relations, automata, invariants, and proofs. |
| Rust systems engineer | Forces clarity about ownership, types, memory, concurrency, error handling, and zero-cost abstractions. |
| JS/TS backend engineer | Compares Spring with Node, npm, TypeScript, Express/Fastify/NestJS, async IO, and ecosystem speed. |
| Skeptical reviewer | Attacks vague learning, tutorial dependency, version cargo culting, and "I know it because I watched it" illusions. |

Chosen thesis:

> Study Spring Boot as the Java incarnation of general backend algebra: objects are state carriers, interfaces are contracts, dependency injection is graph construction, REST is resource transformation over HTTP, databases are relations plus constraints, transactions preserve invariants, security controls identity and capability, and tests are executable proof obligations.

## Source Anchors

Use these as primary references while studying:

- Java Spring Boot roadmap PDF: local source listed above.
- Spring Boot Reference: https://docs.spring.io/spring-boot/reference/index.html
- Spring Framework Reference: https://docs.spring.io/spring-framework/reference/
- Spring Security Reference: https://docs.spring.io/spring-security/reference/
- Spring Data JPA Reference: https://docs.spring.io/spring-data/jpa/reference/
- Java Tutorials - Generics: https://docs.oracle.com/javase/tutorial/java/generics/index.html
- Java API docs: https://docs.oracle.com/en/java/javase/
- The Rust Programming Language: https://doc.rust-lang.org/book/
- Rust standard library: https://doc.rust-lang.org/std/
- MDN JavaScript Guide: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide
- TypeScript Handbook: https://www.typescriptlang.org/docs/handbook/intro.html
- Node.js Learn: https://nodejs.org/learn
- npm Docs: https://docs.npmjs.com/
- Docker Docs: https://docs.docker.com/get-started/
- GitHub Actions Docs: https://docs.github.com/en/actions
- Testcontainers for Java: https://java.testcontainers.org/
- OpenTelemetry Docs: https://opentelemetry.io/docs/

Evidence note: the roadmap phase order and topic clusters come from the local PDF. The ecosystem anchors above are primary documentation sources checked on 2026-05-12. When coding, prefer the version used by the project you create instead of assuming every current doc feature is available.

## 12-Week Intensive Plan

| Week | Roadmap focus | Meta-pattern focus | Deliverable |
| --- | --- | --- | --- |
| 1 | Java syntax, data types, methods, arrays | Types as sets; functions as mappings; arrays as indexed finite sequences | Console drills plus a typed calculator and data transformer. |
| 2 | OOP, collections, modern Java, exceptions | Objects as state machines; interfaces as contracts; errors as alternate control paths | Library Management System and CSV Data Processing Pipeline. |
| 3 | IntelliJ, Maven, Gradle, Git/GitHub | Build graphs, dependency graphs, version history as a DAG | Multi-module Maven project and Git workflow simulation. |
| 4 | IoC, DI, ApplicationContext, component scanning | Dependency injection as graph construction | Notification service with swappable implementations. |
| 5 | Spring beans, scopes, lifecycle, profiles, configuration | Runtime object lifecycle; environment as configuration function | Profile-driven configurable Spring Boot app. |
| 6 | REST fundamentals, HTTP, controllers | REST as resource algebra over protocol operations | Task Management API with clean HTTP semantics. |
| 7 | Validation, error handling, OpenAPI, versioning | Contracts, preconditions, postconditions, and error envelopes | Product API with validation, documented errors, search, filters, and pagination. |
| 8 | Relational databases, SQL, schema design | Relations, keys, normalization, indexes, joins | Blog schema with SQL queries and relation diagrams. |
| 9 | JPA/Hibernate, Spring Data JPA, transactions, migrations | Object-relational mapping; invariants across state transitions | Blog or inventory backend with Flyway and transaction tests. |
| 10 | Security, Spring Security, JWT, OAuth2/OIDC | Identity, authentication, authorization, capabilities, boundaries | JWT auth system and OAuth2 resource-server exercise. |
| 11 | Unit and integration testing | Tests as executable proof obligations | 80 percent coverage target on one prior API using JUnit, Mockito, MockMvc, and Testcontainers. |
| 12 | Docker, CI/CD, deployment, observability | Reproducible environments; pipelines; feedback loops | Dockerized service, GitHub Actions pipeline, health/metrics/logging demo. |

Weekly rhythm:

- 10 hours: read official docs and make notes.
- 15 hours: implement project work.
- 8 hours: compare Java/Spring with Rust and JS/TS.
- 5 hours: write tests and debug.
- 2-7 hours: refactor, document, and explain out loud.

## Phase-by-Phase Concept Matrix

### Phase 1 of 7 - Java Fundamentals

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Java syntax and basics: variables, data types, operators, expressions, control flow, loops, methods, parameters, arrays | You are learning the grammar for giving the computer tiny exact instructions. | Types are sets of possible values. Expressions map inputs to outputs. Control flow is path selection through a program graph. | Java uses declared types, class-based files, methods, primitive/reference values, and arrays with fixed length. Spring still depends on this grammar under every annotation. | Rust also has explicit types, but ownership and borrowing are part of the grammar of safe use. Arrays have fixed size; `Vec<T>` is the growable sequence. | JavaScript is dynamic at runtime; TypeScript adds a static model before runtime. Arrays are growable objects, and type checks are erased after compilation. | Build a typed calculator, then rewrite it as tiny pure methods. | Thinking syntax is "basic" and therefore unimportant; weak syntax becomes weak debugging. |
| Object-oriented programming: classes, objects, constructors, encapsulation, inheritance, `super`, polymorphism, abstract classes, interfaces, access modifiers | A class is a blueprint. An object is a built thing. A method is what the thing can do. | Objects are state plus operations. Interfaces are contracts. Polymorphism is one call shape with multiple implementations. | Java centers OOP: services, controllers, repositories, DTOs, entities, and configuration objects are all class-shaped. Spring uses interfaces heavily for loose coupling. | Rust has structs and enums for data, impl blocks for behavior, and traits for contracts. It avoids inheritance as a main modeling tool. | JS uses prototypes and classes; TS interfaces describe shapes. NestJS uses decorators and classes in a Spring-like style. Express/Fastify are often function-first. | Model books, members, loans, and borrowing rules as Java classes and interfaces. | Overusing inheritance when composition or an interface would make the model cleaner. |
| Collections framework: `List`, `ArrayList`, `LinkedList`, `Set`, `HashSet`, `TreeSet`, `Map`, `HashMap`, `TreeMap`, `Queue`, `Deque`, iterators, `Collections` utility | Different containers are good at different chores: ordered bag, unique bag, lookup table, waiting line. | Collections are data structures with complexity tradeoffs: sequence, set, map, queue, tree, hash table, iterator. | Java APIs constantly return collections: request DTO lists, database results, repository pages, grouped business data. | Rust uses `Vec`, `HashSet`, `BTreeSet`, `HashMap`, `BTreeMap`, `VecDeque`, and iterator adaptors with ownership rules. | JS has `Array`, `Set`, `Map`, plus library helpers. TS can type the contents but runtime collections stay JS objects. | Implement search, grouping, sorting, and duplicate removal for the library app. | Picking collections by habit instead of by access pattern and complexity. |
| Modern Java features: lambdas, functional interfaces, Stream API, Optional, method references, `var` | You can pass behavior around like a value and build data pipelines. | Functions are first-class mappings. Streams compose map/filter/reduce. Optional models "value may be absent." | Spring code uses lambdas in configuration, streams in transformation, `Optional` in repositories, and functional interfaces in callbacks. | Rust closures, iterators, `Option<T>`, and pattern matching make the same ideas more explicit and allocation-conscious. | JS functions are naturally first-class. Arrays have `map`, `filter`, and `reduce`; promises and async functions extend this style. TS types function signatures. | Read CSV transactions, filter, map, reduce, group, and summarize. | Treating streams as magic loops and making them unreadable or stateful. |
| Exception handling: `try`, `catch`, `finally`, checked/unchecked exceptions, custom exceptions, try-with-resources, exception chaining | When something goes wrong, choose whether to recover, report, or stop. | Errors are alternate paths in the control-flow graph. Resources need deterministic cleanup. | Java has exceptions as a core error mechanism. Spring turns exceptions into HTTP responses through handlers and advice. | Rust prefers `Result<T, E>` and `?`, making error paths visible in the type. Panics are for unrecoverable cases. | JS uses `throw`, `try/catch`, rejected promises, and `async/await`. TS does not type thrown errors by default. | Add custom exceptions and resource-safe file reading to the CSV pipeline. | Catching too broadly, swallowing errors, or hiding root causes. |

Phase projects:

- Library Management System: prove OOP, collections, and exception handling.
- Data Processing Pipeline: prove streams, lambdas, file IO, and transformation thinking.

### Phase 2 of 7 - Development Environment and Tools

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| IDE mastery: project setup, structure, shortcuts, navigation, refactoring tools, debugging, Git integration | Your IDE is a map, microscope, and surgery kit for code. | Codebases are graphs of symbols, files, call edges, and transformations. Debugging is state inspection over time. | IntelliJ understands Java/Spring annotations, beans, Maven/Gradle structure, refactors, and debugger frames. | Rust Analyzer gives strong symbol navigation because Rust's compiler model is strict; debugging often pairs with tests and logs. | VS Code/WebStorm plus TS language server provide navigation, refactors, npm scripts, and debugger support. | Navigate an unfamiliar Spring project and trace controller -> service -> repository. | Using the IDE as a text editor and missing its graph intelligence. |
| Maven: POM structure, dependencies, repositories, build lifecycle, plugins, profiles, multi-module projects | Maven is a recipe book for building and fetching ingredients. | Build systems are dependency graphs plus ordered phases. | Spring Boot projects often use Maven for dependencies, packaging, tests, profiles, and plugin-driven builds. | Cargo combines dependency manager, build runner, test runner, and package metadata in `Cargo.toml`. | npm/pnpm/yarn manage packages and scripts; build pipelines may use Vite, tsup, webpack, or framework CLIs. | Create `core`, `api`, and `utils` modules and wire dependencies deliberately. | Copying POM snippets without understanding transitive dependency effects. |
| Gradle: `build.gradle`, dependencies, tasks, Groovy/Kotlin DSL, wrapper | Gradle is a programmable build pipeline. | Tasks form a directed acyclic graph; plugins add nodes and edges. | Many Spring teams use Gradle for faster, flexible builds and custom tasks. The wrapper pins the build tool version. | Cargo is less programmable but more standardized; custom build scripts exist through `build.rs`. | JS build tools are highly programmable but fragmented; package scripts often orchestrate many tools. | Mirror the Maven project in Gradle and compare build mental models. | Treating Gradle like Maven syntax with different punctuation. |
| Git and GitHub: commands, branching, merge vs rebase, pull requests, conflicts, `.gitignore` | Git is a time machine for code, and GitHub is a collaboration surface. | Commit history is a DAG. Merge and rebase are graph transformations. Conflicts are incompatible edits to the same state. | Java teams use Git for reviews, release branches, CI triggers, and production traceability. | Rust projects tend to rely heavily on CI for formatting, clippy, tests, and semver discipline. | JS projects often have many generated/build artifacts, so `.gitignore` and lockfile discipline matter a lot. | Simulate feature branches, conflict resolution, PR review, and a clean history. | Treating Git as just upload/download instead of as a reasoning tool for change. |

Phase projects:

- Multi-Module Maven Project: prove build graph understanding.
- Git Workflow Simulation: prove branch, PR, conflict, and ignore-file discipline.

### Phase 3 of 7 - Spring Core and Spring Boot

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Inversion of Control: problem solved, IoC container, `ApplicationContext`, bean definition, component scanning | Instead of every object building its own helpers, a central organizer wires the team. | Dependency injection is graph construction. Nodes are objects; edges are dependencies; the container builds the graph. | Spring's `ApplicationContext` discovers bean definitions and creates managed objects. Component scanning turns annotated classes into graph nodes. | Rust usually wires dependencies explicitly with structs, traits, constructors, and generics. There is no default runtime container. | Express/Fastify often wire manually. NestJS uses a DI container closer to Spring. TS decorators can describe graph wiring. | Build a notification app with email, SMS, and push implementations. | Saying "Spring magic" instead of naming the object graph and lifecycle. |
| Dependency Injection: constructor injection, setter injection, field injection, `@Autowired`, `@Qualifier`, `@Primary`, circular dependencies | Give objects what they need from the outside instead of making them hunt for it. | DI reduces coupling by making dependencies explicit graph edges. Qualifiers choose among multiple valid edges. | Constructor injection is preferred because required dependencies are clear and testable. Field injection hides requirements. | Rust constructors take dependencies directly; traits and generics encode replaceable behavior. Circular ownership requires deliberate `Rc`, `Arc`, `Weak`, or redesign. | Node apps often pass dependencies manually or use containers in NestJS/Inversify. TypeScript interfaces disappear at runtime, so tokens/classes may be needed. | Swap notification providers through configuration with no service-code change. | Creating circular dependencies that reveal muddled responsibility boundaries. |
| Spring beans: `@Component`, `@Service`, `@Repository`, bean scopes, lifecycle, `@PostConstruct`, `@PreDestroy`, `@Configuration`, `@Bean`, conditional beans | A bean is an object Spring owns, names, creates, shares, and eventually cleans up. | Lifecycle is a state machine: define -> construct -> initialize -> use -> destroy. Scope controls identity and reuse. | Stereotype annotations mark roles. Singleton beans are shared by default. Configuration classes create explicit beans. Conditional beans change the graph by environment or classpath. | Rust values have lexical lifetimes and RAII cleanup through `Drop`; sharing requires explicit ownership types. | JS objects are usually created by modules/factories. NestJS providers have scopes and lifecycle hooks similar to Spring. | Create singleton and prototype beans and log lifecycle events. | Storing request-specific mutable state in singleton beans. |
| Spring Boot fundamentals: starters, auto-configuration, `application.properties`/YAML, profiles, `@Value`, `@ConfigurationProperties`, DevTools | Spring Boot pre-assembles common setups so you do not hand-wire every bolt. | Auto-configuration is rule-based graph completion. Configuration is a function from environment to behavior. | Starters pull dependency sets. Auto-configuration activates when conditions match. Profiles select environment-specific values. Typed config binds external values into objects. | Rust frameworks usually use explicit config structs and builders; feature flags affect compile-time inclusion. | JS frameworks often use `.env`, config modules, package plugins, and runtime conventions. NestJS has config modules. | Build dev/test/prod profiles with typed configuration. | Fighting auto-configuration without first inspecting what it configured. |

Phase projects:

- Dependency Injection Showcase: prove IoC, DI, interfaces, and configuration.
- Configurable Application: prove profiles and typed external configuration.

### Phase 4 of 7 - Building REST APIs

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| REST fundamentals: principles, resources, URIs, HTTP methods, status codes, headers, content negotiation | An API is a menu of named things. HTTP verbs say what you want to do with each thing. | REST is resource algebra over protocol operations. Methods are transformations; status codes are outcomes; headers are metadata. | Spring MVC maps HTTP requests to controller methods and serializes Java DTOs to responses. | Rust web frameworks like Axum and Actix map routes to typed handlers. Extractors make request parts explicit. | Express/Fastify route handlers are flexible; NestJS controllers are annotation/decorator based. Fetch APIs consume endpoints from clients. | Design a Task API with correct URIs, methods, status codes, and headers. | Treating every operation as `POST /doThing` instead of modeling resources. |
| Spring MVC controllers: `@RestController`, `@Controller`, `@RequestMapping`, method annotations, `@PathVariable`, `@RequestParam`, `@RequestBody`, `@ResponseBody`, `ResponseEntity`, `@RequestHeader`, `@CookieValue` | Controllers are translators between web requests and Java methods. | Boundary adapters map external protocol data into internal function calls. | Annotations bind URI parts, query params, bodies, headers, and cookies to method parameters. `ResponseEntity` controls status, headers, and body. | Rust extractors parse path/query/body/header into typed values; the handler signature exposes the boundary. | Express uses `req` and `res` objects. NestJS decorators mirror Spring's annotation style. | Implement CRUD endpoints with path variables, query filters, request bodies, and response statuses. | Putting business logic in controllers instead of delegating to services. |
| Request validation: Bean Validation, `@Valid`, `@Validated`, built-in constraints, custom validators, validation groups, error messages | Validation is a bouncer at the door: bad input does not enter the party. | Validation encodes preconditions. A precondition is what must be true before a function or state transition runs. | Spring integrates Bean Validation into controller inputs and DTOs. Custom validators encode domain rules. | Rust can validate through constructors, smart types, `Result`, and crates like validator; invalid states can be made unrepresentable. | JS uses runtime validators such as Zod, Yup, Joi, class-validator, or framework pipes. TS types alone do not validate runtime input. | Add validation to Task and Product APIs with helpful field errors. | Believing TypeScript or Java types alone protect external JSON input. |
| Exception handling: `@ExceptionHandler`, `@ControllerAdvice`, custom exceptions, error response DTOs, Problem Details, logging errors | Give users a clear receipt when something fails. | Error envelopes are stable output contracts for failed state transitions. | Global advice keeps controller code clean and maps exceptions to consistent HTTP responses. Problem Details standardizes error shape. | Rust handlers often return `Result<Response, AppError>` and implement conversions into HTTP responses. | Express uses error middleware; NestJS uses exception filters; TS APIs often use typed error classes plus runtime mapping. | Build one error response shape for validation, not found, conflict, and internal errors. | Leaking stack traces or returning vague `"something went wrong"` errors. |
| API documentation: OpenAPI specification, springdoc-openapi, Swagger UI, annotations, versioning strategies | Documentation is a map that clients can click and test. | API specs are machine-readable contracts between producers and consumers. Versioning manages contract evolution. | Springdoc can generate OpenAPI from controllers and annotations. Swagger UI gives interactive exploration. | Rust has crates such as utoipa and aide for OpenAPI generation, but integration varies by framework. | JS/TS uses OpenAPI generators, Swagger tooling, tRPC, GraphQL schemas, or framework metadata. | Publish Swagger UI for Task API and verify another person can call it from docs alone. | Treating docs as decoration instead of as a client contract. |

Phase projects:

- Task Management API: prove REST, validation, errors, and documentation.
- E-commerce Product API: prove pagination, filtering, search, DTOs, and response shaping.

### Phase 5 of 7 - Data Persistence

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Relational database fundamentals: tables, rows, columns, primary keys, foreign keys, normalization, indexes, SQL CRUD, joins | A database is a set of connected spreadsheets with rules. | Relations are sets of tuples. Keys enforce identity. Foreign keys encode graph edges. Indexes trade write cost for read speed. | Spring apps still need SQL literacy even when JPA hides some details. Schema design shapes all persistence behavior. | Rust can use SQLx for checked queries, Diesel for ORM-like modeling, or SeaORM. Ownership is not the hard part; schema truth is. | JS/TS uses Prisma, TypeORM, Drizzle, Knex, Sequelize, or raw SQL. Runtime validation and migrations need discipline. | Design tables for users, posts, comments, tags, and post_tags. | Learning ORM first and never learning the SQL it generates. |
| JPA and Hibernate: `@Entity`, `@Id`, generation strategies, field mappings, relationships, lazy vs eager loading, cascading | JPA lets Java objects have database shadows. | ORM maps between object graphs and relational tables. The mismatch is structural: graphs vs relations. | Entities represent persisted state. Relationships model associations. Lazy loading delays fetching; cascading propagates operations. | Rust ORMs are usually more explicit and less magical. Query boundaries and ownership are harder to hide. | Prisma gives a schema-first ORM. TypeORM feels closer to JPA decorators. Drizzle is more SQL-shaped. | Build Blog entities and compare generated SQL for common queries. | Accidentally triggering N+1 queries through lazy relationship traversal. |
| Spring Data JPA: `JpaRepository`, query derivation, `@Query`, JPQL/native SQL, pagination, sorting, projections, DTOs, specifications | Repositories are vending machines for common data operations. | A repository is an abstraction over query functions. Pagination is slicing an ordered set. Projections are partial views. | Spring Data derives queries from method names and supports explicit JPQL/native SQL for control. | Rust repository patterns are manually designed; SQLx query functions make boundaries explicit. | Prisma client generates typed query functions. TypeORM repositories and query builders resemble the Java mental model. | Add derived queries, JPQL, pagination, projections, and specifications to Product API. | Writing clever derived method names when a clear query would be easier to read. |
| Transactions: ACID, `@Transactional`, propagation, isolation, rollback rules, read-only transactions | A transaction is a promise that a group of changes succeeds together or not at all. | Transactions preserve invariants across state transitions. Isolation controls what concurrent observers can see. | Spring uses proxies around transactional methods. Propagation and rollback rules decide transaction boundaries. | Rust DB libraries expose transactions as values/closures; the type system can make transaction scope explicit. | JS ORMs expose transaction callbacks or clients; async errors and connection reuse require care. | Implement stock movement updates that cannot create negative inventory. | Putting `@Transactional` on a method that is never called through the Spring proxy. |
| Database migrations: why migrations matter, Flyway basics, scripts, schema version control, rollback strategies | Migrations are the database's version history. | Schema evolution is ordered state transformation. Each migration changes the set of valid database states. | Flyway/Liquibase version schema changes and run them repeatably across environments. | Rust projects use refinery, sqlx migrations, SeaORM migrations, or raw scripts. | Prisma Migrate, Drizzle Kit, TypeORM migrations, and Knex migrations play similar roles. | Add Flyway migrations for every schema change in the inventory app. | Editing old migrations after they have already been applied somewhere. |
| Query optimization and N+1 awareness | Asking the database one question at a time can become a thousand hidden trips. | Complexity is not only CPU; IO round trips dominate distributed systems. | Use fetch joins, entity graphs, DTO projections, indexes, and query inspection to avoid N+1 and slow paths. | Rust raw SQL often makes query count obvious, but you can still create bad access patterns. | JS ORMs can hide N+1 too; dataloaders and query planning help. | Log SQL and prove a list endpoint uses bounded query count. | Measuring only local happy-path speed and ignoring query count. |

Phase projects:

- Blog Platform Backend: prove relationships, ownership, pagination, transactions, and query optimization.
- Inventory Management System: prove transaction invariants, reporting queries, aggregations, and migrations.

### Phase 6 of 7 - Security

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Security fundamentals: authentication vs authorization, OWASP Top 10, BCrypt, HTTPS/TLS, CORS, security headers | First prove who someone is, then decide what they may do. | Security models identity, capability, trust boundaries, threat surfaces, and safe state transitions. | Spring apps must separate login identity from endpoint permission. Passwords are hashed, not encrypted for later reading. | Rust can reduce memory safety bugs, but auth, crypto choices, and web threats still need correct design. | JS/TS web apps face the same web threats, plus package supply-chain risk and browser/server boundary mistakes. | Write a threat model for the Task API before adding login. | Thinking "we use JWT" means "we are secure." |
| Spring Security basics: filter chain, `SecurityFilterChain` bean, authentication manager, `UserDetailsService`, password encoders, method security with `@PreAuthorize` | Requests pass through guards before reaching the room. | Security filters are a pipeline. Each filter transforms or rejects the request state. | Spring Security is built around a filter chain and explicit configuration. Method security protects service methods too. | Rust frameworks often use middleware/extractors; auth state is passed explicitly to handlers. | Express/Fastify use middleware. NestJS uses guards, interceptors, decorators, and strategies. | Secure endpoints by role and add service-level method checks. | Debugging endpoints without understanding filter order. |
| JWT authentication: header, payload, signature, creating and validating tokens, access/refresh tokens, storage, filter implementation, expiration and refresh | A JWT is a signed badge. Anyone can read it, but only the signer can make a valid one. | Tokens are bearer capabilities. Signature verifies integrity, not secrecy. Expiration bounds risk. | Spring Security can validate JWTs as a resource server or through a custom filter. Refresh tokens need careful storage and revocation design. | Rust has JWT crates, but correctness depends on algorithm choice, claims validation, clock handling, and storage. | Node has many JWT libraries; middleware makes setup easy but insecure defaults are common. | Implement register, login, refresh, logout/invalidation, and role-based endpoints. | Storing sensitive secrets in JWT payload or accepting tokens without validating claims. |
| OAuth2 and OpenID Connect: flows, authorization code flow, resource server, social login, OIDC basics | OAuth lets another trusted party help with login or authorization without sharing passwords. | OAuth is delegated authorization. OIDC adds identity claims on top. Scopes model limited capabilities. | Spring Security has OAuth2 client and resource-server support. Authorization Code Flow is the normal modern web flow. | Rust ecosystem support exists but is less batteries-included; explicit protocol handling is common. | JS/TS has mature provider SDKs and Auth.js/Passport-style integrations; frontend token storage risks matter. | Build an API that validates an external provider token and checks scopes. | Mixing up authentication, authorization, OAuth, OIDC, scopes, roles, and claims. |
| CORS, TLS, headers, and token storage | The browser, network, and API each need separate boundary rules. | Boundaries are trust transitions. CORS is browser policy, TLS is transport security, headers are protocol constraints. | Spring can configure CORS and security headers; TLS is often terminated at a proxy or platform. | Rust services still depend on reverse proxies, TLS config, and correct browser policy. | JS apps must coordinate frontend origin, cookies, local storage/session storage, SameSite, and backend CORS. | Document token storage choices for browser, CLI, and server-to-server clients. | Fixing CORS by allowing everything in production. |

Phase projects:

- JWT Authentication System: prove password hashing, login, refresh, logout/invalidation, and RBAC.
- OAuth2 Resource Server: prove token validation, scopes, and external identity-provider integration.

### Phase 7 of 7 - Testing and Production

| Roadmap concept | ELI5 intuition | Meta-math / CSE pattern | Java / Spring interpretation | Rust comparison | JS / TS ecosystem comparison | Practice task | Common trap |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Unit testing: JUnit 5, assertions, lifecycle, Mockito, argument matchers, verifying interactions | A unit test asks one small piece, "Do you keep your promise?" | Tests are executable proof obligations. Assertions are predicates over behavior. Mocks control neighboring nodes in the call graph. | JUnit and Mockito test services in isolation, verify edge cases, and protect refactors. | Rust has built-in tests and strong compile-time checks, but behavior still needs tests. Mocking often uses traits or test doubles. | JS/TS uses Jest, Vitest, Node test runner, Sinon, Testing Library, and framework-specific tools. | Add service tests for all task/product/blog business rules. | Mocking so much that the test proves the mock, not the code. |
| Integration testing: `@SpringBootTest`, `@WebMvcTest`, `@DataJpaTest`, Testcontainers, MockMvc, test configuration | Integration tests check that the pieces actually talk to each other. | Integration tests verify composition: graph nodes, boundaries, serialization, persistence, and configuration. | Spring test slices let you test controller, repository, or full app layers. Testcontainers gives realistic infrastructure. | Rust integration tests often spin up services or use test DBs directly; Axum/Actix handlers can be tested with request objects. | JS/TS uses Supertest, Playwright API tests, Dockerized services, and framework test modules. | Test controller validation, repository queries, and PostgreSQL behavior with Testcontainers. | Relying only on mocks for database and HTTP boundary behavior. |
| Docker: basics, Dockerfile, multi-stage builds, Docker Compose, environment variables, health checks | Docker packages your app with the world it needs to run. | Containers make environment state reproducible. Images are layered build artifacts. Compose models a local service graph. | Spring Boot apps can be packaged as jars or container images. Compose can run app plus database. | Rust produces small static-ish binaries and can make lean images, but native dependencies still matter. | Node images need dependency-layer discipline, lockfiles, and production dependency pruning. | Containerize one API and run it with PostgreSQL through Compose. | Shipping dev dependencies, secrets, or huge images without noticing. |
| CI/CD and deployment: GitHub Actions, build/test pipelines, Docker registry, Kubernetes intro, cloud platforms, environment management | A pipeline is a robot that builds, tests, packages, and ships the app the same way every time. | CI/CD is an automated state machine over commits, artifacts, environments, and approvals. | Java pipelines run Maven/Gradle tests, build artifacts/images, and deploy with environment-specific configuration. | Rust CI usually runs fmt, clippy, tests, builds, and cross-compile or container packaging. | JS/TS CI must manage package managers, lockfiles, build steps, linting, tests, and runtime images. | Create a GitHub Actions pipeline that tests, builds an image, and publishes or stores it. | Manual deployment steps that are not captured in the pipeline. |
| Observability: structured logging, Spring Boot Actuator, Micrometer metrics, health endpoints, distributed tracing basics | Observability is how your app tells you what happened after it leaves your laptop. | Observability maps runtime behavior into signals: logs, metrics, traces, and health states. | Spring Boot Actuator exposes health and management endpoints. Micrometer provides metrics. Tracing follows requests across services. | Rust uses tracing/log crates and OpenTelemetry integrations; explicit spans often produce good signal. | Node uses pino/winston, OpenTelemetry, framework middleware, metrics clients, and platform logs. | Add health, metrics, structured request logs, and one trace/span path. | Adding logs only after production breaks. |
| Microservices readiness | A microservice is a small deployable system with its own boundaries, data, and failure modes. | Microservices are distributed graphs of state machines. Network calls introduce partial failure. | Spring Boot is common for microservices, but boundaries, data ownership, and observability matter more than the framework. | Rust is attractive for high-performance services but increases explicit systems work. | Node is common for IO-heavy services and fast iteration, but runtime discipline matters. | Split one bounded capability only after the monolith has clear module boundaries. | Turning every class into a service and calling it architecture. |

Phase projects:

- Fully Tested API: prove unit, controller, repository, integration, and container-backed tests.
- Dockerized Microservice: prove Docker, Compose, CI/CD, deployment, logs, health, and metrics.

## Meta-Pattern Glossary

Read this section repeatedly. It is the "make me smarter over time" layer.

| Term | ELI5 first | More technical version | Why it matters |
| --- | --- | --- | --- |
| Type | A label for what kind of thing a value is. | A set of possible values plus allowed operations. | Prevents invalid operations and clarifies API contracts. |
| Function | A machine that turns inputs into output. | A mapping from a domain to a codomain, possibly with effects. | Helps separate pure computation from IO and mutation. |
| Object | A thing with memory and behaviors. | Encapsulated state plus methods over that state. | Core of Java and Spring modeling. |
| Interface | A promise about what methods exist. | A behavioral contract implemented by multiple concrete types. | Enables substitution and test doubles. |
| Trait | Rust's interface-like contract. | A set of required behavior that can be statically or dynamically dispatched. | Makes Rust polymorphism explicit and composable. |
| Generic | A recipe with a blank type slot. | Parametric polymorphism: code abstracted over type parameters. | Lets you write reusable code without losing type safety. |
| Ownership | Who is responsible for a value. | Rust's compile-time discipline for memory and aliasing. | Explains why Rust APIs feel explicit and safe. |
| Borrowing | Letting someone use a value without owning it. | References with rules for aliasing and mutation. | Prevents data races and use-after-free. |
| Lifetime | How long a reference is valid. | A static constraint tying reference validity to scope. | Makes hidden memory assumptions visible. |
| Collection | A container for many values. | Data structure with complexity and ordering semantics. | Drives performance and correctness. |
| Iterator | A controlled way to visit values one by one. | Abstraction over sequential access and lazy transformation. | Powers streams, pipelines, and memory-efficient processing. |
| Closure / Lambda | A tiny function you can pass around. | Function value that may capture surrounding environment. | Central to modern Java, Rust, and JS. |
| Optional / Option | A box that may be empty. | Sum type or wrapper encoding presence/absence. | Avoids uncontrolled null behavior. |
| Exception / Result | A way to represent failure. | Exception is control transfer; `Result` is typed success/failure data. | Shapes error handling philosophy. |
| Dependency | Something a component needs to do its job. | A graph edge from one unit to another. | Makes architecture visible. |
| Dependency injection | Giving components their dependencies from outside. | External graph construction for loose coupling and testability. | Foundation of Spring Core. |
| Bean | A Spring-managed object. | Runtime container-managed instance with scope and lifecycle. | Explains Spring object ownership. |
| Scope | How widely an object is shared. | Identity and lifetime policy for instances. | Prevents shared mutable-state bugs. |
| REST resource | A named thing the API exposes. | Stable URI-addressed abstraction over state. | Makes APIs predictable. |
| HTTP method | The verb of a web request. | Protocol operation with semantics such as safe, idempotent, or mutating. | Drives correct API behavior. |
| DTO | A shaped message object. | Boundary data structure for transport, not domain truth. | Keeps internal models decoupled from API shape. |
| Validation | Checking input before trusting it. | Enforcing preconditions at system boundaries. | Prevents invalid state and security bugs. |
| Relation | A table-like set of facts. | Set of tuples over named attributes. | Core of SQL and relational databases. |
| Primary key | A row's identity badge. | Unique identifier for a tuple in a relation. | Makes updates and references precise. |
| Foreign key | A pointer from one table to another. | Referential integrity constraint across relations. | Preserves graph consistency in relational form. |
| Normalization | Avoid copying the same fact everywhere. | Schema design that reduces redundancy and update anomalies. | Improves consistency. |
| Index | A shortcut for finding rows. | Auxiliary data structure optimizing reads at write/storage cost. | Explains query performance. |
| Transaction | All-or-nothing group of changes. | State transition preserving invariants under concurrency and failure. | Essential for data correctness. |
| ACID | Four promises about transactions. | Atomicity, Consistency, Isolation, Durability. | Names the database safety model. |
| Migration | A versioned database change. | Ordered transformation of schema state. | Makes production data evolution controlled. |
| Authentication | Proving who you are. | Establishing identity. | First half of access control. |
| Authorization | Deciding what you may do. | Capability or policy decision over an identity. | Second half of access control. |
| JWT | A signed token badge. | Compact claims format with integrity protection. | Common for stateless APIs, but easy to misuse. |
| OAuth2 | Letting another system authorize access. | Delegated authorization protocol. | Powers third-party integrations. |
| OIDC | Login identity on top of OAuth2. | Authentication layer using ID tokens and claims. | Powers "login with" flows. |
| Unit test | Test one small promise. | Local executable predicate over a unit's behavior. | Fast feedback. |
| Integration test | Test that pieces work together. | Composition proof across boundaries. | Catches wiring and infrastructure bugs. |
| Container | Packaged runtime environment. | Isolated process environment built from image layers. | Makes deployment reproducible. |
| CI/CD | Robot build and delivery line. | Automated state machine from commit to artifact/deployment. | Reduces manual release risk. |
| Observability | The app explaining itself at runtime. | Logs, metrics, traces, and health signals for inference. | Lets you debug production. |

## Capstone Projects

### Capstone A - Java/Spring Production API

Build a Spring Boot backend for a personal learning tracker.

Required features:

- Users, study plans, roadmap phases, tasks, notes, and progress.
- REST API with validation, global error handling, and OpenAPI docs.
- PostgreSQL persistence with Flyway migrations.
- Spring Data JPA repositories with pagination, projections, and at least one custom query.
- Transactional progress update that preserves an invariant, such as "completed tasks cannot exceed total tasks."
- JWT authentication with roles: learner and admin.
- Unit, controller, repository, and integration tests with Testcontainers.
- Docker Compose for app plus database.
- GitHub Actions pipeline for build and tests.
- Actuator health, structured logs, and at least one metric or trace.

Meta-pattern proof:

- Draw the dependency graph.
- Draw the database relation graph.
- List all invariants.
- List all trust boundaries.
- Explain one request from HTTP input to database transaction to response.

### Capstone B - Rust Comparison Slice

Rebuild a narrow vertical slice of Capstone A in Rust using a web framework such as Axum or Actix Web.

Required comparison:

- Route handler signatures vs Spring controller methods.
- Traits vs Java interfaces.
- `Result<T, E>` vs Java exceptions.
- Config structs/builders vs Spring Boot externalized configuration.
- SQLx/Diesel/SeaORM style vs Spring Data JPA.
- Explicit ownership and lifetimes vs JVM garbage collection.

Do not attempt full feature parity. The goal is contrast, not duplicate labor.

### Capstone C - JS/TS Comparison Slice

Rebuild the same narrow vertical slice in TypeScript using either Express/Fastify or NestJS.

Required comparison:

- Middleware/guards/pipes vs Spring filters/controllers/validation.
- Zod/class-validator/runtime validation vs Java Bean Validation.
- Prisma/Drizzle/TypeORM vs Spring Data JPA.
- Promise/async error handling vs Java exceptions and Rust `Result`.
- npm scripts and package ecosystem vs Maven/Gradle and Cargo.

Do not skip runtime validation. TypeScript types do not prove incoming JSON is valid.

## Verification Checklist

Use this checklist every Sunday.

Learning proof:

- Can I explain the ELI5 version without jargon?
- Can I name the formal CS x math pattern?
- Can I implement the Java/Spring version?
- Can I describe how Rust would force different choices?
- Can I describe how JS/TS would make the same idea more dynamic or more ergonomic?
- Can I name one production failure mode?
- Can I write or point to a test that proves the behavior?

Roadmap coverage proof:

- Java fundamentals covered: syntax, OOP, collections, modern Java, exceptions.
- Tooling covered: IntelliJ, Maven, Gradle, Git/GitHub.
- Spring Core covered: IoC, DI, beans, Spring Boot configuration.
- REST covered: HTTP, MVC controllers, validation, errors, docs, versioning.
- Persistence covered: SQL, JPA/Hibernate, Spring Data JPA, transactions, migrations, N+1.
- Security covered: fundamentals, Spring Security, JWT, OAuth2/OIDC.
- Production covered: unit tests, integration tests, Docker, CI/CD, deployment, observability.

Evidence proof:

- Do not rely on tutorial memory for framework behavior; check official docs.
- Do not claim "secure" without a threat model and tests.
- Do not claim "production-ready" without deployment, health checks, logs, metrics, and rollback thinking.
- Do not claim "I know it" until you can build it, compare it, and explain the failure modes.

## Open Questions

- Which Spring Boot major version will your actual capstone use? The docs may show multiple maintained versions; choose one and pin it in the project.
- Do you want the Rust comparison to use Axum, Actix Web, or another framework?
- Do you want the JS/TS comparison to use NestJS for Spring-like structure, or Fastify/Express for lower-level contrast?
- Which database should be the default for all capstones: PostgreSQL, MySQL, or H2 only for tests?
- Will the final capstone be portfolio-oriented, interview-oriented, or production-deployment-oriented?

