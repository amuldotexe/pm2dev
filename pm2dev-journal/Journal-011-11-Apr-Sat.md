# Journal 011 - 11-Apr-Sat

``` text

It is a beautiful morning here in BLR



```

## Video Ideas

``` text
- Tech twitter does not talk about their mid 30s journey, everything is about young people
- 
```



##

# Unclassified Notes


### Notes from a paper released on Mythos - Graph structures are on the move

Yes, but mostly at the **reasoning/model** layer, not at the **storage/runtime** layer.

The paper is **“Scaling Latent Reasoning via Looped Language Models”** ([arXiv abstract](https://arxiv.org/abs/2510.25741), submitted **October 29, 2025**, revised **November 17, 2025**). Its core claim is that models can get better at reasoning by doing **iterative computation in latent space** during pretraining, with **adaptive depth / early exit**, instead of relying mainly on explicit chain-of-thought text. The authors argue the gains come more from **knowledge manipulation** than from storing more knowledge, and that this approach avoids some **context-length bloat** from long CoT outputs. Sources: [abstract](https://arxiv.org/abs/2510.25741), [paper HTML](https://ar5iv.labs.arxiv.org/html/2510.25741).

How that connects to our conversation:

- **Strongly related to our “context engineering” discussion.**
  We kept talking about not dumping raw code and instead giving the model the right structured substrate. This paper strengthens that worldview: better reasoning is not just “more tokens” or “longer CoT,” but better internal manipulation of the right facts.

- **Strongly related to Parseltongue’s value proposition.**
  Parseltongue is trying to turn code into a compact, traversable graph artifact. This paper suggests future models may become better at **composing and manipulating retrieved structure** once they have it. That is very aligned with a public-interface graph / smart-context system.

- **Related to our build-time vs walk-time split.**
  The paper’s looped model does heavy internal iterative computation, then can **early-exit** on easier cases. That rhymes with our idea that build time does expensive graph compilation once, while walk time should stay cheap and adaptive. It is not the same mechanism, but the philosophy is similar: **pay compute where it matters, not everywhere**.

- **Related to our graph-walking intuition.**
  Their “knowledge manipulation” framing is close to what multi-hop code understanding feels like:
  - who calls this
  - what does that reach
  - what is the public boundary
  - what is the blast radius  
  That is basically structured fact composition over a graph.

- **Not directly related to Iggy / CSR / CSC / persisted graph stores.**
  The paper says nothing about graph storage layouts, code indexing, or traversal runtimes. So it does **not** answer our storage questions.

My practical takeaway for our project:

- If models get better at **latent multi-hop reasoning**, then Parseltongue becomes **more valuable**, not less.
- Why: we do not need the model to memorize the whole repo; we need it to receive a **small, faithful, traversable representation** and reason over it well.
- That means our work on:
  - graph snapshots
  - CSR/CSC-style traversal
  - build-time compilation
  - compact context delivery  
  still makes sense.

The cleanest summary is:

**This paper is not about how to store the graph, but it is very much about why a compact, traversable graph could be the right thing to hand to a stronger reasoning model.**