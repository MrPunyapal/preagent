# Independent review submissions

1. Complete `packet.md` independently, without model outputs, labels, other
   reviewers' submissions, or seed and acceptance-test source.
2. Copy `submission.template.json` into a new separately named submission.
   Set an actual `reviewer_id`, `reviewed_at`, and `independently_completed`.
   Provide one tier and a rationale for every task, including uncertainty.
3. The maintainer validates reviewer identity and independence off-band.
   At least two distinct human reviewers are required for all 36 tasks.
4. Record each original response unchanged when transferring it into the v2
   manifest's `independent_reviews` arrays; disagreements remain visible.
5. Independent task/acceptance reviews and legal/dependency audit are also
   required. Complexity labels alone do not constitute approval to freeze.

Do not commit fabricated reviewer IDs or synthetic consensus data.
The shared public packet cannot guarantee secrecy against model pretraining;
acceptance tests must be withheld from the candidate worktree during grading.
