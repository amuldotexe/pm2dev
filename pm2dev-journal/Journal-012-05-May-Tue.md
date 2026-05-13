


# Notes from various Udemy courses

```


```

# Random Notes from my own code

``` text

  Title:
     How to understand Apache Iggy PR #2815 like an engineer, not like a syntax parser

     Short answer first — Minto Pyramid Principle

     Main point:
     PR 2815 is not mainly “about MongoDB” and not mainly “about Rust.”
     It is about one hard systems question:

     How do you write streaming messages into MongoDB without lying about success when failures happen halfway through?

     Everything else in the PR exists to answer that.

     The 3 big ideas under that are:

     1. Identity:
        Every message needs a deterministic unique ID so retries do not create duplicates.

     2. Partial failure honesty:
        If a batch insert partly succeeds and partly fails, the connector must classify that correctly.

     3. Liveness under retries:
        Recoverable failures should not kill the sink forever.

     That is the real structure of the PR.

     ASCII pyramid

                     [Main claim]
        This PR makes MongoDB sink writes replay-safe and failure-aware

              /                 |                 \
             /                  |                  \
      [1] Deterministic   [2] Partial failure   [3] Keep running after
          message identity     classification        recoverable errors

           |                       |                     |
      composite _id         unordered bulk insert   retry-safe behavior
      duplicate tolerance   duplicate != fresh fail sink stays live

     Now let’s do ELI5.

     ELI5 version

     Imagine a mailroom.

     You have a machine whose job is:
     - take letters from a conveyor belt
     - put them into boxes in a warehouse

     Now the machine can fail in annoying ways:
     - it may insert some letters, then jam
     - it may not know whether the last few letters got inserted
     - someone may press “retry” and send the same letters again

     If the machine is dumb:
     - duplicates appear
     - it reports “all failed” when half succeeded
     - it stops forever after a temporary issue

     PR 2815 is basically teaching the machine to behave like this:

     1. Stamp each letter with a permanent identity.
     2. If retried, recognize “I already stored this one.”
     3. If half the box made it in, say “partial success,” not “total failure.”
     4. If the warehouse burps temporarily, try again instead of dying.

     That is the PR.

     PART 1 — What problem was the author actually solving?

     A beginner often explores code like this:
     - What files changed?
     - What functions were added?
     - What does this syntax mean?

     An ideal engineer starts differently:

     Question 1:
     What contract is this component trying to uphold?

     For this PR, the contract is approximately:

     “Iggy messages should be delivered into MongoDB with at-least-once semantics, while making retries safe and handling partial failures honestly.”

     That one sentence tells you what to look for.

     Not syntax.
     Semantics.

     The right first questions would have been:

     1. What is the unit of work?
        - one message?
        - one batch?

     2. What counts as success?
        - all inserted?
        - some inserted?
        - duplicates acceptable?

     3. What happens on retry?
        - duplicate documents?
        - overwrite?
        - reject?

     4. How does it distinguish “already inserted before” from “new failure”?

     5. What metadata gets stored with each message?

     6. What do the tests prove about real failure modes?

     That is the ideal exploration frame.

     ASCII review map

     [Connector contract]
             |
             v
     [Input message stream]
             |
             v
     [Transform each message into MongoDB document]
             |
             +--> how is _id built?
             +--> what fields are stored?
             +--> what payload shape is used?
             |
             v
     [Insert batch into MongoDB]
             |
             +--> ordered or unordered?
             +--> what happens on duplicate?
             +--> what happens on timeout?
             +--> what happens on partial success?
             |
             v
     [Return status honestly]
             |
             +--> success
             +--> partial / retry-safe duplicate case
             +--> real failure

     PART 2 — How to explore the codebase without knowing Rust

     If you do not know Rust, do not start from syntax.

     Start from file roles.

     In this PR, the important files are conceptually:

     1. README
        Why it exists:
        - tells you intended behavior in English
        - often states guarantees and limitations

     2. main sink implementation
        Why it exists:
        - shows how one message becomes one MongoDB document
        - shows batch write behavior
        - shows error classification

     3. integration tests
        Why they exist:
        - tell you what the author was afraid of
        - tell you the real edge cases
        - often explain the design better than the implementation

     Ideal exploration order

     Step 1: README first
     Ask:
     - What does the connector claim?
     - What semantics does it promise?
     - What limitations does it admit?

     From this PR, the README matters because it tells you:
     - at-least-once delivery semantics
     - deterministic composite _id
     - duplicate replay tolerance
     - known limitation around partial commit uncertainty on network timeout

     That already gives you the whole story.

     Step 2: Find identity creation
     Ask:
     - How does one message become one MongoDB document?
     - What is the _id?

     This PR uses a deterministic composite _id like:

     stream:topic:partition:message_id

     That is the centerpiece.

     Why?
     Because retry safety depends on stable identity.

     If the same message is retried, it generates the same _id.
     MongoDB then rejects it as duplicate instead of storing it twice.

     ASCII

     same message retried twice
             |
             v
     same deterministic _id
             |
             v
     MongoDB says:
     "already have this"
             |
             v
     safe replay, no double insert

     Without this:

     same message retried twice
             |
             v
     random/new id each time
             |
             v
     MongoDB stores both
             |
             v
     duplicate data bug

     This is the single most important design move in the PR.

     Step 3: Find batch insert mode
     Ask:
     - Does it insert one by one or in bulk?
     - Does one failure stop the rest?

     This PR uses unordered batch insertion:
     insert_many(...).ordered(false)

     Translate that into English:

     “Try the whole batch; don’t stop at the first bad document.”

     Why this matters:
     If one document is a duplicate, you do not want the rest of the valid messages to be blocked.

     ASCII

     ordered = true
     [A][B][C][D]
          X duplicate
     result: stop at B, maybe C and D never attempted

     ordered = false
     [A][B][C][D]
          X duplicate
     result: A, C, D can still succeed

     This is a very systems-minded decision.
     It improves throughput and makes duplicates less destructive.

     Step 4: Find duplicate error handling
     Ask:
     - Does the code treat duplicate key as a fatal bug?
     - Or as evidence of retry/idempotency?

     In this PR, duplicate key error 11000 is special.

     The logic is roughly:
     - if all write errors are duplicate-key errors
     - and there is no write-concern error
     - then this is treated as replay-safe rather than fresh corruption

     That is subtle and very important.

     Why?
     Because in distributed systems, “duplicate” often means:
     “we retried after uncertainty, but the first attempt already succeeded.”

     So duplicate is not automatically a failure.
     Sometimes it is proof that idempotency is working.

     ASCII

     Attempt 1 ---> insert succeeded
     Network burps
     Sink is unsure

     Attempt 2 ---> same message, same _id
     MongoDB ---> duplicate key

     Naive interpretation:
     "something failed"

     Correct interpretation:
     "good, retry did not create a second copy"

     That is a core systems insight.

     Step 5: Read tests as requirements
     Ask:
     - What failure stories are being simulated?

     The MongoDB integration tests are not decoration.
     They are the best explanation of the PR.

     Ideal non-Rust reading method:
     Ignore syntax.
     Look for test names and scenarios.

     You want to extract stories like:
     - duplicate replay should not count as fresh failure
     - partial writes should not be reported as full success
     - sink should remain alive after recoverable errors
     - metadata and payload should land in expected shapes

     That tells you what correctness means here.

     PART 3 — The real conceptual model of the sink

     Think of the sink as 4 stages:

     ASCII pipeline

     [Iggy message]
          |
          v
     [Create MongoDB document]
          |
          +--> _id = stream:topic:partition:message_id
          +--> metadata fields
          +--> payload representation
          |
          v
     [Bulk insert into MongoDB]
          |
          +--> unordered
          +--> collect write errors
          |
          v
     [Classify result]
          |
          +--> success
          +--> duplicate-only replay-safe case
          +--> real failure / uncertainty

     Now let’s unpack each one.

     A. Input message
     A stream system emits messages.
     Each message already has meaningful identity information:
     - stream
     - topic
     - partition
     - message id / offset-ish identity

     The sink should preserve enough of that structure to make storage replay-safe.

     B. Document creation
     Each incoming message becomes a MongoDB document.

     The sink stores:
     - the message payload
     - metadata like stream/topic/partition
     - timestamps
     - checksum
     - origin-related metadata
     - deterministic _id

     This is not just “store bytes.”
     It is “store bytes plus enough context to debug, trace, and deduplicate.”

     C. Batch insertion
     The sink inserts batches for efficiency.

     But batching creates the hardest bug class:
     partial success.

     Example:

     Batch = 10 messages
     MongoDB inserts first 7
     Then timeout / network issue / write error happens

     Now what is true?
     - 7 may already be in DB
     - 3 may not be
     - the sink may not know fully

     This is the heart of the PR.

     D. Result classification
     A correct system must avoid these lies:

     Lie 1:
     “Everything failed”
     when actually some messages already made it.

     Lie 2:
     “Everything succeeded”
     when some may not have made it.

     Lie 3:
     “Duplicate means corruption”
     when duplicate only reflects retry safety.

     This PR is trying to be more honest than naive code.

     PART 4 — The key design insight: deterministic identity

     If I had to teach only one thing from this PR, it would be this:

     In distributed systems, retries are normal.
     Retries are only safe if identity is stable.

     The composite _id is the backbone:

     _id = stream:topic:partition:message_id

     Why each part matters:

     - stream:
       distinguishes one broad source from another

     - topic:
       distinguishes category/channel within stream

     - partition:
       distinguishes shard/lane of ordering

     - message_id:
       distinguishes the individual message within that lane

     Together, they make “this exact message” addressable.

     ASCII identity card

     +--------------------------------------------+
     | stream     = orders                        |
     | topic      = paid                          |
     | partition  = 3                             |
     | message_id = 18442                         |
     +--------------------------------------------+
     | _id = orders:paid:3:18442                  |
     +--------------------------------------------+

     Why not random UUID at sink time?

     Because retries would generate a different UUID.
     Then the same logical message would be stored twice.

     Stable identity is what turns retry from danger into safety.

     PART 5 — Why unordered bulk insert matters

     A beginner might think:
     “If duplicates are possible, just fail the batch.”

     But that is operationally bad.

     Suppose 100 messages are being inserted.
     2 are duplicates from retry.
     98 are fresh.

     If insertion stops at first duplicate:
     - throughput suffers
     - harmless duplicates poison useful work

     Using unordered bulk insert says:
     “Try everything. Let good writes succeed even if some are duplicates.”

     ASCII

     Batch:
     [M1][M2][M3][M4][M5]

     Suppose M2 and M4 are duplicates.

     ordered = true
     M1 success
     M2 duplicate -> stop
     M3/M4/M5 maybe never attempted

     ordered = false
     M1 success
     M2 duplicate
     M3 success
     M4 duplicate
     M5 success

     This matches the system’s actual intent better.

     PART 6 — The hardest part: partial failure

     This is where the PR becomes genuinely interesting.

     In simple app code, people think:
     - success or failure

     In systems code, reality is often:
     - some committed
     - some didn’t
     - and the writer is not fully sure

     That is why the review mattered.

     The core review question is not:
     “Is the code clean?”

     It is:
     “Does the connector say something truthful about ambiguous outcomes?”

     Example scenario

     1. Sink sends batch of 10 docs.
     2. MongoDB writes 8.
     3. Network error occurs before confirmation is fully observed.
     4. Sink must decide what to report/do next.

     Possible bad behaviors:
     - mark all 10 failed, causing duplicate replay chaos
     - mark all 10 succeeded, causing silent loss of 2
     - crash and never recover

     The PR tries to navigate this honestly.

     ASCII ambiguity box

             batch of 10
                 |
                 v
        +-------------------+
        | MongoDB got some? |
        +-------------------+
             /        \
           yes        no
           |           |
     maybe 8 wrote   maybe 0 wrote
           |
     writer cannot know perfectly
           |
           v
     must retry safely and report carefully

     This is why the README’s “known limitation” matters.
     Good engineering includes naming what cannot be perfectly known.

     That honesty is a strength, not a weakness.

     PART 7 — What the tests are really teaching

     If you want to think like a reviewer, convert tests into invariants.

     Here are the likely invariants this PR cares about:

     1. Same logical message should map to same MongoDB _id
     2. Retrying same message should not create a second stored copy
     3. Duplicate replay should not be treated as a fresh data-integrity bug
     4. Partial writes should not be misreported as total clean success
     5. Recoverable errors should not permanently kill the sink
     6. Stored documents should preserve payload + metadata shape correctly

     ASCII invariant board

     [Identity]
     same input -> same _id

     [Replay safety]
     retry -> duplicate key, not duplicate data

     [Honesty]
     partial outcome != pretend full success

     [Liveness]
     recoverable issue != sink dies forever

     [Traceability]
     stored doc keeps enough metadata to debug later

     That is how an ideal engineer “reads” the codebase.
     Not token by token.
     Invariant by invariant.

     PART 8 — How a non-Rust reader should read the implementation

     Pretend every function is a black box with a job.

     You do not need Rust syntax.
     You need function purpose.

     When looking at the sink source, ask:

     1. Where is document creation?
        Translate to:
        “Where do they decide what one MongoDB row looks like?”

     2. Where is _id built?
        Translate to:
        “Where do they decide message identity?”

     3. Where is batch insert called?
        Translate to:
        “Where do they hand the work to MongoDB?”

     4. Where are write errors inspected?
        Translate to:
        “Where do they decide duplicate vs real failure?”

     5. Where is retry/continuation behavior decided?
        Translate to:
        “Where do they decide whether sink keeps going?”

     You are reading roles, not syntax.

     Useful mental translation table

     Rust-looking thing          Real question

     struct / enum               What kinds of states/data exist?
     fn                          What job is being done?
     match                       What cases are being distinguished?
     Result / error              What can fail?
     insert_many                 Is this bulk write?
     ordered(false)              Does one failure block the rest?
     DuplicateKey / 11000        Are retries being treated idempotently?

     This is the ideal way to not get trapped by unfamiliar language.

     PART 9 — Why reviewers cared so much

     This PR was large, but reviewers likely did not care most about size.
     They cared about semantic traps.

     The dangerous traps were:

     Trap 1: accidental duplicate creation
     Solved by:
     - deterministic composite _id

     Trap 2: batch poisoned by one duplicate
     Solved by:
     - unordered insert

     Trap 3: duplicate replay counted as fresh failure
     Solved by:
     - special duplicate-key classification

     Trap 4: partial writes hidden under a fake clean result
     Solved by:
     - explicit handling and documentation of limitations

     Trap 5: recoverable failures kill connector
     Solved by:
     - liveness-minded error handling and test coverage

     So if you ask, “what makes this PR good?”
     The answer is:

     It turns invisible distributed-systems failure modes into explicit design choices.

     PART 10 — The ideal investigation path, step by step

     If I were teaching someone how to understand this from scratch, I’d say:

     Step 1: Read the PR title and README
     Goal:
     Understand the promise in English.

     You should leave with:
     “This is a MongoDB sink with at-least-once semantics and replay-safe identity.”

     Step 2: Ignore syntax and inspect data shape
     Goal:
     Find how one message becomes one MongoDB document.

     You should leave with:
     “Each stored document includes payload, metadata, and a deterministic _id.”

     Step 3: Find the write call
     Goal:
     Understand batch semantics.

     You should leave with:
     “They use unordered bulk insertion so duplicates do not block unrelated writes.”

     Step 4: Find duplicate handling
     Goal:
     Understand retry/idempotency semantics.

     You should leave with:
     “Duplicate key can be an expected replay artifact, not necessarily a fresh failure.”

     Step 5: Find acknowledged limitations
     Goal:
     Understand honesty boundaries.

     You should leave with:
     “There are cases like network timeout after partial commit where perfect certainty is impossible.”

     Step 6: Read integration tests last
     Goal:
     See what the authors considered scary.

     You should leave with:
     “The tests encode the real contract: replay safety, partial-failure behavior, and connector liveness.”

     That is the ideal path.

     PART 11 — One-sentence interpretations of the major choices

     Composite _id
     “Make retries land on the same identity.”

     Unordered insert_many
     “Let harmless duplicates fail locally without blocking the whole batch.”

     Duplicate key special handling
     “Treat replay evidence differently from fresh corruption.”

     Metadata storage
     “Keep enough context to debug provenance and behavior later.”

     README limitation notes
     “Do not overclaim precision where distributed systems are inherently ambiguous.”

     Integration tests
     “Prove semantics under ugly real-world cases, not just happy-path inserts.”

     PART 12 — What a junior engineer should learn from 2815

     The big lesson is this:

     Good systems code is not mainly about writing code that works on success.
     It is about defining what the truth is when the world fails halfway.

     ASCII summary

     Happy path:
     message -> doc -> MongoDB -> success

     Real world:
     message -> doc -> maybe inserted -> maybe timed out -> maybe retried
                                   |
                                   v
                           correctness depends on:
                           identity + classification + honesty

     That is why this PR matters.

     Final Minto-style conclusion

     Top-level answer:
     PR 2815 is best understood as a correctness-and-reliability PR for a MongoDB sink, not as a Rust feature PR.

     Supporting points:
     1. It creates deterministic message identity via composite _id.
     2. It uses unordered bulk inserts so duplicates do not poison whole batches.
     3. It classifies duplicate-key outcomes as replay-safe when appropriate.
     4. It tries to remain honest about partial-write ambiguity.
     5. It uses tests to encode liveness and failure semantics.

     ELI5 ending:
     The author taught the sink to say:
     “I know which message is which, I won’t store the same one twice on retry, I won’t pretend a messy batch was perfectly clean, and I won’t fall over just because the network sneezed.”

     If you want, next I can turn this into one of these:
     1. a one-page study note
     2. a “how to review systems code without knowing syntax” checklist
     3. a line-by-line pseudocode walkthrough of mongodb_sink/src/lib.rs in plain English

```