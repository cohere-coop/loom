# Loom

Loom is a development-environment process manager with grand ambitions.

Right now, it can only run a set of processes as a group.
Think `foreman`, but without `foreman`'s chdir problem.

Eventually, it will provide log aggregation and management features, among other things.
Its goal is to make development environments observable and operable in *parallel ways* to production environments, rather than in radically different ways.
This will help users develop observability & operability instincts that bridge the development vs production gap... and therefore the product-engineering vs systems-operations gap.

## Installation

Installation is a hand-compiled trashfire for now.
Sorry.

```
[install rustc and cargo if you haven't already]
git clone https://github.com/wecohere/loom.git
cd loom
cargo build --release
ln -s target/release/loom /somewhere/in/your/PATH
```

## Usage

Loom is configured using TOML.

Here's an example `loomfile.toml`:

```toml
[web-client]
command = "npm run dev --prefix web-client"

[api]
command = "bin/rails server"
directory = "./api"
```

Top-level keys (like `web-client` and `api` in the example) name processes; this is used to label their output.
On the second level, the `command` key is **required** and indicates which command to run.
The second-level `directory` key is **optional** and indicates the working directory to run the command in.
(This is *intended* to set the working directory correctly from the perspective of language version managers like rvm, chruby, nvm, etc.
If it doesn't, it's a bug; please report it!)

## Contributing

PLEASE DO!
But also, we have a code of conduct and we take it seriously.

We will do our best to provide clear, kind, and actionable feedback to all contributors within a reasonable amount of time.
However, Cohere is still levelling up on our Rust skills.
We may fall down on the "clear," "actionable," and "timely" parts of that goal sometimes.
(We will try our darnedest to not fall down on "kind.")

On that note, one kind of contribution that is EXTREMELY WELCOME: are you an experienced Rust developer?
Do you want to give *us* clear, kind, and actionable feedback on how to fix the parts of our code that are SUPER MESSY because we gave up on fighting the borrow checker?
WE WOULD LIKE THAT A LOT. 